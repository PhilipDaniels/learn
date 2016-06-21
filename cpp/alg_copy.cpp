#include <set>
#include <vector>
#include "alg_print.hpp"

using namespace std;

// A demonstration of the fundamental algorithm, copy.

int main(int argc, char *argv[])
{
    vector<int> src { 1, 2, 3, 4, 1, 2 };
    vector<int> dest;

    // front_inserter or back_inserter.
    copy(src.begin(), src.end(), back_inserter(dest));
    print_elements(dest, "dest: ");

    // The only inserter that works for associative containers.
    set<int> s;
    copy(src.begin(), src.end(), inserter(s, s.begin()));
    print_elements(s, "set: ");

    return 0;
}
