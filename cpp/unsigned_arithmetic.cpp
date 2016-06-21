#include <iostream>

using std::cout;
using std::endl;

int main(int argc, char *argv[])
{
    unsigned int u = 10;
    int i = -42;
    
    cout << "Adding two ints is fine, this prints -84 because i is -42" << endl;
    cout << i + i << endl << endl;

    cout << "Adding two unsigned ints of value 10 also works ok" << endl;
    cout << u + u << endl << endl;

    cout << "But mixing signed and unsigned causes errors because " << endl
	 << "the signed value of -42 is first converted to unsigned" << endl;
    cout << i + u << endl; // prints 4294967264 on a machine with 32 bit ints

    cout << endl
	 << "Guidance" << endl
	 << "  * Don't mix signed and unsigned ints in arithmetic expressions." << endl
	 << "  * Don't use unsigned types as loop counters because they cannot be negative." << endl
	 << "    and this can cause your loops to never terminate." << endl;
}
