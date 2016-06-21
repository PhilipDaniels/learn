#include <array>
#include <iostream>

using std::array;
using std::cout;
using std::endl;

int main(int argc, char *argv[])
{
    // Defines an array with 3 elements, numbered 0..2.
    int a[3];

    // Size is inferred if you do this.
    int ints[] = { 1, 2, 3, 4 };
    size_t num_elements = sizeof(ints)/sizeof(*ints);
    cout << "num_elements in ints = " << num_elements << endl;

    // This won't compile, the number of initializers must match.
    //int ints2[4] = { 1, 2, 3, 4, 5 };

    // This is actually a std::initializer_list<int> !!
    // And initializer lists are constants.
    auto cints = { 1, 2, 3, 4 };

    // foo[0] is 11, the rest are guaranteed to be 0.
    int foo[10] = { 11 };

    // This zero-initializes everything.
    int foo2[10] = { };

    // std::arrays can be copied (makes an independent copy).
    array<int, 4> a1 = { 1, 2, 3, 4 };
    array<int, 4> a2 = a1;
    a2[0] = 20;

    cout << "a1[0] = " << a1[0] << ", a2[0] = " << a2[0] << endl;

    cout << "Advice: use std::array for fixed size arrays.\n"
         << "        use std::vector for re-sizable arrays." << endl;
    return 0;
}
