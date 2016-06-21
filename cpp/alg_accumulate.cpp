#include <functional>
#include <iostream>
#include <numeric>
#include <vector>

using namespace std;

int main(int argc, char *argv[])
{
    vector<int> src { 1, 2, 100, 200, 300 };
    int sum = accumulate(src.begin(), src.end(), 0);
    cout << "sum = " << sum << "\n";

    int product = accumulate(src.begin(), src.end(),
                             1, multiplies<int>());
    cout << "product = " << product << "\n";

    return 0;
}
