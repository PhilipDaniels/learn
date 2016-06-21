#include <iostream>
#include <vector>

using std::cout;
using std::endl;
using std::vector;

int main(int argc, char* argv[])
{
    vector<int> v = { 2, 3, 6, 7, 10 };

    // When using iterators:
    //   1. Use != end() to indicate termination, it is more general than <.
    //   2. Always use pre-increment (that is a general C++ principle).
    //   3. Prefer const iterators if not changing the elements.
    for (auto i = v.cbegin(); i != v.cend(); ++i)
    {
	cout << *i << ", ";
    }
    cout << endl;
}
