#include <iostream>
#include <iterator>
#include <numeric>
#include <vector>
#include "alg_print.hpp"

using namespace std;

int main(int argc, char *argv[])
{
    vector<int> src { 1, 2, 3, 4, 5, 6 };
    vector<int> dest;

    // partial_sum is used to calculate running totals.
    // It is the opposite of adjacent_difference.
    partial_sum(src.begin(), src.end(), back_inserter(dest));
    print_elements(dest);

    // Get back to the original.
    adjacent_difference(dest.begin(), dest.end(),
                        ostream_iterator<int>(cout, " "));
    cout << "\n";

    return 0;
}
