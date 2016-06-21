#include <algorithm>
#include <functional>
#include <string>
#include <vector>
#include "alg_print.hpp"

using namespace std::placeholders;
using namespace std;

int main(int argc, char *argv[])
{
    vector<int> src { 1, 2, 3, 4, 5 };
    vector<int> dest;

    // Transform is like for_each but with a different destination.
    // multiplies is a built-in function object.
    transform(src.begin(), src.end(),
              back_inserter(dest),
              bind(multiplies<int>(), _1, 10));

    print_elements(dest);

    // You can also call transform with two sequences, which
    // makes it act like the functional "zip".
    vector<string> src2 { "aa", "bb", "cc", "dd", "ee" };
    vector<string> dest2;

    transform(src.begin(), src.end(),
              src2.begin(), // 2nd end is implied.
              back_inserter(dest2),
              [] (int x, string s) { return s + "-" + to_string(x); });

    print_elements(dest2);

    return 0;
}
