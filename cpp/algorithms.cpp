#include <algorithm>
#include <iostream>
#include <iterator>
#include <string>
#include <vector>

using std::cout;
using std::endl;
using std::string;
using std::vector;

template<class InputIterator>
void dump(InputIterator beg, InputIterator end)
{
    while (beg != end)
    {
        cout << *beg++ << " ";
    }
    cout << endl;
}

int main(int argc, char *argv[])
{
    // Principle: algorithms never execute container operations, so they never
    // shrink or grow the container (it is necessary to use erase to remove
    // "extra" items).

    // Virtually all algorithms are of the following form (everthing shown is an
    // iterator). beg and end are the input range. Some algorithms take only a
    // single iterator for their second range: they assume that the 2nd
    // container is at least as big as the first.
    //     alg(beg, end, ...)
    //     alg(beg, end, dest, ...)     << alg writes to dest
    //     alg(beg, end, beg2, ...)     << alg reads from dest
    //     alg(beg, end, beg2, end2, ...)

    // Algorithms that take a predicate and that do not take other arguments
    // are typically overloaded:
    // unique(beg, end);
    // unique(beg, end, comp);

    // Algorithms that take an element value typically have an "_if" version
    // which avoids overloading ambiguities:
    // find(beg, end, val);
    // find_if(beg, end, pred);

    // Rearranging algorithms by default write back to the original range. They
    // provide a second version "_copy" that writes to a specified destination.
    // reverse(beg, end);
    // reverse_copy(beg, end, dest);

    // Finally, some algorithms provide both _copy and _if versions:
    // remove(beg, end, val);
    // remove_if(beg, end, pred);
    // remove_copy_if(beg, end, dest, pred);



    // Algorithms do not range-check write operations, they assume the
    // destination container is large enough. To grow a container, use
    // back_inserter().
    vector<int> v;
    auto bi = back_inserter(v);
    fill_n(bi, 10, 42);

    // Many algorithms take a unary or binary predicate. Lambdas are useful for
    // these.
    vector<string> words { "hello", "microscope", "c++", "C++" };
    sort(words.begin(), words.end(),
         [] (const string& lhs, const string& rhs) { return lhs.size() < rhs.size(); });
    dump(words.begin(), words.end());


    // Demonstration of copying to a new container.
    vector<int> nums { 10, 20, 30 };
    vector<int> reverse_nums;
    auto bin = back_inserter(reverse_nums);
    reverse_copy(nums.begin(), nums.end(), bin);
    dump(reverse_nums.begin(), reverse_nums.end());

    return 0;
}
