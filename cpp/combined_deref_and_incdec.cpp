#include <iostream>
#include <vector>

using std::cout;
using std::endl;
using std::vector;

int main(int argc, char *argv[])
{
    // The postfix versions of -- and ++ are used when we want to use the
    // current value of a variable and increment it in a single compound
    // expression.

    // *pv++ means the same as *(pv++) because postfix increment has a higher
    // precedence than *. Since pv++ yields a copy of the current value and then
    // increments, this means we print the whole vector.
    vector<int> v = { 1, 2, 3, 4 };
    auto pv = v.begin();
    while (pv != v.end())
	cout << *pv++ << endl;

    cout << "Second loop is in error - skips the first and prints end()" << endl;
    pv = v.begin();
    while (pv != v.end())
	cout << *++pv << endl;

    return 0;
}
