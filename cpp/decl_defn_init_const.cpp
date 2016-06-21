#include <iostream>

using std::endl;
using std::cout;

// This is a definition - it allocates storage, it is "the real object".
// Because it is at global scope it is initialized to 0.
int a_definition;

// This is a declaration - it makes the name known to the program.
// This will typically appear in a header file, and the corresponding definition
// will occur in the corresponding cpp file, to ensure that storage for the
// variable is allocated once, but known multiple times.
extern int a_declaration;

// This is a definition too - the extern is ignored and it is equivalent to
// "int another_definition = 42". This will usually give a compiler warning
// because of the useless "extern".
extern int another_definition = 42;


int getSize() { return 12; }


int main(int argc, char *argv[])
{
    cout << "another_definition = " << another_definition << endl;

    int a;         // Not initialized, has undefined value.
    int b(4);      // Initialized to 4.
    int c = 5;     // Initialized to 5. Not an assignment because this object is being created.
    int d{6};      // Initialized to 6.
    cout << a << " " << b << " " << c << " " << d << endl << endl;

    cout << "Initialization is not assignment.\n"
	    "Initialization happens when an object is given a value upon creation.\n"
	    "Assignment obliterates an object's current value with a new one.\n";

    /////////////////////////////////////////////////////////////////////////

    // Reference declarations (the same rules apply to pointers).
    // References are always bound to the same object (pointers can be re-aimed).
    int e1 = 1024, e2 = 2048, e3 = e1;   // e3 is ok, e1 is initialized first.
    int &r1 = e1, r2 = e2;               // WARNING: r2 is an int!
    int &r3 = e1, &r4 = e2;              // This is the correct way.

    int *p1 = &e1;
    int **p2 = &p1;
    cout << endl;
    cout << "**p2 = " << **p2 << "\n"
	 << " *p2 = " <<  *p2 << "\n"
	 << "  p2 = " <<   p2 << "\n\n";

    cout << "Guidance:\n"
	 << "  AVOID: int* p;          // Can be confusing with multiple definitions.\n"
	 << "   GOOD: int *p, *q;\n"
	 << "   GOOD: int* p;          // Limit to one definition per line.\n\n"
	 << "Read declarations from right to left:\n"
	 << "         int *p;          // Pointer. Can change p and *p.\n"
         << "   const int *p;          // Pointer to const. Can change p only.\n"
	 << "         int *const p;    // Const pointer. Can change *p only.\n"
	 << "   const int *const p;    // Const pointer to const. Can't change anything.\n"
	 << "   int **&rp2 = p2;       // rp2 is a reference to an int **.\n\n"
	 << "n.b. Pointers to const may only be 'pretending' to point to a const\n"
	 << "object, they may in fact be pointing to a non-const object that\n"
	 << "may be changed from somewhere else in the program.\n";

    // You cannot have pointers to references because references are not objects.
    // But you can have references to pointers.
    int *&rp1 = p1;
    int **&rp2 = p2;


    /////////////////////////////////////////////////////////////////////////
    // const.
    // Constants must be given a value at the time of definition.
    const int bufSize = 512;
    const int bufSize2 = getSize();

    // Using constants in multiple files. (Lippman, p60).
    // A const definition is normally local to the file that it is in ("internal
    // linkage"), this means that you can write "const int foo = 2" in a header
    // and include it in multiple files without any problems. In fact, the
    // lines:
    //    const int bufSize = 12;
    //    static const int bufSize = 12;
    // are identical in meaning (static means internal linkage).
    // n.b. const having internal linkage is different to C.
    //
    // However, if you want to initialize a constant using a runtime value you
    // must do this:
    //   in file.cpp:  extern const int bufSize = func();
    //   in file.hpp:  extern const int bufSize;
    //
    // A third approach is to use a namespace:
    //   namespace CurlConstants {
    //     const int CurlTimeout = 0xFF;
    //   }


    // Pointers (references work exactly the same way) to const combinations.
    int p, q;

    // Read from right to left.
    int *w = &p;                // Pointer. Can change w and *w.
    *w = 42;
    w = &q;

    const int *x = &p;          // Pointer to const. Can change x only.
    // *x = 42;
    x = &q;

    int *const y = &p;          // Const pointer. Can change *y only.
    *y = 42;
    //y = &q;

    const int *const z = &p;    // Const pointer to const. Can change nothing.
    //*z = 42;
    //z = &q;


    // Lippman, p63, $2.4.3, Top-level const.
    // Top-level const means the object itself is const.
    //     Object may mean an int, or a pointer to reference.
    // Low-level const means the thing pointed to or referred to is const.
    //     Low-level const appears in the base type of compound types such
    //     as pointers or references.

    // Only pointers can have both types of const.

    // When we copy an object, top-level consts are ignored; copy an object
    // does not change the source, so top-level const is irrelevant.
    const int ci = 2;
    int nci = ci;

    // Low-level const is never ignored during copying. We can convert a non-
    // const to const, but not the other way round.
    const int* llc = &ci;
    // int *nllc = llc;      // No, can't convert 'const int *' to 'int *'.
    llc = w;                 // Ok, can convert 'int *' to 'const int *'.
    llc = z;                 // Ok, can convert 'const int *const' to 'const int *'.
}
