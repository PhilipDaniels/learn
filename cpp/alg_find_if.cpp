#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

bool big_enough(int x)
{
    return x > 100;
}

struct BigEnough
{
    bool operator()(int x) const { return x > 4; };
};

void dump(vector<int>::iterator pos, vector<int>::iterator end)
{
    if (pos != end)
    {
        cout << "Found it: " << *pos << "\n";
    }
    else
    {
        cout << "Not found.\n";
    }
}

int main(int argc, char *argv[])
{
    vector<int> src = { 1, 2, 3, 5, 101, 200, 4, 4 };

    // Using a function.
    auto pos = find_if(src.begin(), src.end(), big_enough);
    dump(pos, src.end());

    // Using a lambda.
    pos = find_if(src.begin(), src.end(),
                  [] (int x) { return x > 150; });
    dump(pos, src.end());

    // Using a function object (deprecated, prefer lambdas).
    pos = find_if(src.begin(), src.end(), BigEnough());
    dump(pos, src.end());

    return 0;
}
