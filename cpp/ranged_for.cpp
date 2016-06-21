#include <iostream>

using std::cout;
using std::endl;

int main(int argc, char *argv[])
{
    int ints[4] = { 1, 2, 3, 4 };

    // You must have a type, auto is nice.
    for (int i : ints)
        cout << "i = " << i << endl;

    // To mutate, use a reference.
    for (int &i : ints)
        i *= 2;

    cout << endl;
    for (int i : ints)
        cout << "new i = " << i << endl;

    // You can use auto with a reference too.
    cout << endl;
    for (auto &i : ints)
        i *= 2;
    for (int i : ints)
        cout << "new i = " << i << endl;

    return 0;
}
