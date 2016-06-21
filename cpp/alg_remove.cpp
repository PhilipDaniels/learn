#include <algorithm>
#include <vector>
#include "alg_print.hpp"

using namespace std;

int main(int argc, char *argv[])
{
    vector<int> src { 1, 2, 3, 1, 2, 3, 1, 2, 3 };

    // Principle: algorithms never grow or shrink containers
    // (unless you use an inserter).
    print_elements(src, "src is now            : ");
    remove(src.begin(), src.end(), 3);
    print_elements(src, "After call to remove  : ");

    // This is how it should be done - we get back the new end pos.
    vector<int> src2 = { 1, 2, 3, 1, 2, 3, 1, 2, 3 };
    print_elements(src2, "src2 is now          : ");
    auto end = remove(src2.begin(), src2.end(), 3);
    src2.erase(end, src2.end());
    print_elements(src2, "After call to erase  : ");

    // Alternatively, we do not need to call erase() if we just pass the new end
    // around. This can save quite a lot of work.
    vector<int> src3 = { 1, 2, 3, 1, 2, 3, 1, 2, 3 };
    print_elements(src3, "src3 is now          : ");
    end = remove(src3.begin(), src3.end(), 3);
    print_elements(src3.begin(), end, "After call to remove : ");

    // n.b. Many containers provide member functions which do the equivalent
    // operation (remove etc.), provide better performance, *and* actually do
    // the container mutation. list<T>.remove(), for example.
    // Advice: Favour member functions over algorithms.

    // The _if variants take a predicate.
    vector<int> src4 = { 1, 5, 10, 20, 25, 50, 100 };
    print_elements(src4, "src4 is now          : ");
    end = remove_if(src4.begin(), src4.end(),
                    [] (int x) { return x % 4 == 0; });
    print_elements(src4.begin(), end, "After removal of % 4 : ");

    // The _copy_if variants take a predicate and a new destination.
    vector<int> src5 = { 1, 5, 10, 20, 25, 50, 100 };
    vector<int> dest;
    print_elements(src5, "src5 is now          : ");
    remove_copy_if(src5.begin(), src5.end(),
                   back_inserter(dest),
                   [] (int x) { return x % 4 == 0; });
    print_elements(dest, "Dest removal of % 4  : ");

    return 0;
}
