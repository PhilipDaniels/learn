#include <iostream>

using std::cout;
using std::endl;

struct SomeData { int x; int y; int z; };

int main(int argc, char *argv[])
{
    int x;

    // We can get the size of an expression.
    size_t s = sizeof x;
    cout << "size of x == " << s << endl;

    // We can also get the size of a type. This is the sizeof a pointer.
    s = sizeof (&x);
    cout << "size of &x == " << s << endl;

    // Some more examples using a struct.
    SomeData sd;
    s = sizeof sd;
    cout << "size of sd == " << s << endl;
    auto sdp = &sd;
    s = sizeof sdp;
    cout << "size of sdp (a pointer again) == " << s << endl;
    s = sizeof (*sdp);
    cout << "size of *sdp (thing pointed to) == " << s << endl;

    // Size of reference types is the size of the thing referred to.
    SomeData &rsd = sd;
    s = sizeof (rsd);
    cout << "size of rsd (ref to SomeData) == " << s << endl;

    cout << "sizeof (char) is always " << sizeof (char) << endl;

    // sizeof arrays.
    int ia[5];
    cout << "sizeof an array is the size on bytes of the entire array: sizeof a = " << sizeof ia << endl;
    constexpr int num_elements = sizeof ia / sizeof (*ia);
    cout << "We can calculate the number of elements in an array using the formula\n"
	 << "sizeof(a)/sizeof(*a), hence num elements in ia = " << sizeof(ia)/sizeof(*ia) << endl;

    // sizeof returns a constexpr so it can be used in compile time expressions.
    int iacopy[num_elements];

    return 0;
}
