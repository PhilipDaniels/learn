#include <algorithm>
#include <iostream>
#include <vector>
#include "alg_print.hpp"

using namespace std;

int main(int argc, char *argv[])
{
    vector<int> a { 100, 20, 300, 3, 400, 45 };
    print_elements(a, "Initial vector: ");

    auto m = min_element(a.begin(), a.end());
    cout << "min_element = " << *m << "\n";
    m = max_element(a.begin(), a.end());
    cout << "max_element = " << *m << "\n";
    auto mm = minmax_element(a.begin(), a.end());
    cout << "minmax_element = " << *mm.first << "," << *mm.second << "\n";

    sort(a.begin(), a.end());
    print_elements(a, "Sorted vector: ");
    reverse(a.begin(), a.end());
    print_elements(a, "Reversed vector: ");
    sort(a.begin(), a.begin() + 3);
    print_elements(a, "Partially sorted (first 3 elements) vector: ");

    return 0;
}
