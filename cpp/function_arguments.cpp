#include <iostream>
#include <string>

using std::cout;
using std::endl;
using std::string;

void foo(int x) { cout << "first foo" << endl; }
// This is a redefinition. Advice: don't use const on primitive parameter types.
// void foo(const int x) { }
void foo(int &x) { cout << "second foo" << endl; }


// Advice: use references to avoid copies of large objects.
// Add "const" when you don't want to change it.
int countit(const string &s)
{
    int count = 0;
    for (auto c : s)
    {
	if (c == 'i')
	    count++;
    }
    return count;
}

// You can change things using pointers, but references can do
// the same and are considered better (less noisy syntax).
void changeit(int *pi)
{
    *pi = 42;
}

// Ptr-to-ptr: change the thing that p2 points to.
// To change p2 (which is an int*) we need to get a pointer to it,
// which means an int**.
void changeit2(int *p1, int **p2)
{
    *p2 = p1;
}


int main(int argc, char *argv[])
{
    foo(2);

    int a = 3;
    //foo(a);                       // Ambiguous.
    int &b = a;
    //foo(b);                       // Still ambiguous.
    foo(static_cast<int>(a));       // Ok, (though poor style).
    //foo(static_cast<int&>(b));    // Still ambiguous.

    string name("Philip Daniels");
    int c = countit(name);
    cout << "countit returned " << c << endl;

    a = 10;
    changeit(&a);
    cout << "After changeit, a is " << a << endl;

    a = 10;
    int x = 20;
    int *p = &a;
    changeit2(&x, &p);
    cout << "After changeit2, p now points to x. Proof: *p = " << *p << endl;

    cout << "\nAdvice:\n* Don't overload on T and ref-to-T.\n"
	 << "* Use references to avoid copies. Add const if you don't need to change it.\n"
	 << "* Use references to return extra info (like var in C#).\n"
	 << "* Default arguments, inline and constexpr should be specified in a header file."
	 << endl;

    return 0;
}
