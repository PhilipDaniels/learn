#include <cassert>
#include <iostream>

using std::cout;
using std::endl;

int main(int argc, char *argv[])
{
#ifdef NDEBUG
    cout << "NDEBUG is defined, so I will not do any runtime checks." << endl;
    // We can also use the following symbols:
    // __func__ is the name of the function
    // __FILE__ is the name of the file
    // __LINE__ is the current line number
    // __TIME__, __DATE__ is the time and date the file was compiled
#else
    cout << "NDEBUG is NOT defined, so runtime checks will be performed." << endl;
#endif

    // The behaviour of assert() is controlled by the NDEBUG ("no debugging")
    // pre-processor variable. By default NDEBUG is not defined, so assert()
    // performs a runtime check and will stop the program if the condition is
    // false.
    assert(1 == 0);

    // To define NDEBUG, do "#define NDEBUG 1" in some header, or pass it to the
    // compiler, for example "g++ -D NDEBUG assert.cpp -o assert.exe".
    return 0;
}
