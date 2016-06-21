#include <string>
#include <vector>

using std::string;
using std::vector;

class HasPtr
{
    friend void swap(HasPtr& lhs, HasPtr& rhs);
    string* ps;
    int i;

    // Copy-and-swap technique. Lippman, $$ 13.3, p518.
    // We get the compiler to make a copy by passing rhs by value.
    // This technique automatically handles self-assignment and is exception
    // safe. See http://stackoverflow.com/questions/3279543 for another example.
    // n.b. You also need a working copy ctor, dtor and a swap function.
    HasPtr& operator=(HasPtr rhs)
    {
        swap(*this, rhs);
        return *this;
    }
};

inline void swap(HasPtr& lhs, HasPtr& rhs)  // inline, this is an optimization!
{
    using std::swap;
    // All calss to swap are unqualified - so that the compiler will resolve to
    // any class-specifics swaps rather than using the std::swap.
    swap(lhs.ps, rhs.ps);     // swap the pointers, not the strings
    swap(lhs.i, rhs.i);       // swap the int members
}


int main(int argc, char *argv[])
{
    vector<int> v1(10);
    vector<int> v2(20);

    // Excepting std::array, no copies, deletions or insertions are made and is
    // guaranteed to run in constant time.
    std::swap(v1, v2);

    // Classes that manage resources often also define a function named 'swap'.
    // Swap is important for classes that will be used with algorithms that
    // reorder elements, because they call swap when they need to exchange two
    // elements.
    // Swap is never necessary, but can be an optimization.
    // A typical implementation is shown above.

    return 0;
}
