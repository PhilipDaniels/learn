#include <iostream>

using std::cout;
using std::endl;

char foo()
{
    cout << "I am never seen" << endl;
}

int main(int argc, char *argv[])
{
    // decltype never executes its expression.
    decltype(foo()) c = 'a';

    const int ci = 0, &cj = ci;
    decltype(ci) x = 0;          // x is const int
    decltype(cj) y = x;          // y is const int&

    // decltype((var)) is always a reference type, but decltype(var) is a
    // reference type only if var is a reference.
    decltype((ci)) a = ci;      // a is const int&.
    decltype(ci) b = 12;        // b is const int.
}
