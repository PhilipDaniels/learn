#include <algorithm>
#include <iostream>
#include <iterator>
#include <vector>

using namespace std;

struct MyUnaryPred
{
    // You should always declare the operator() method of any predicate to be
    // constant, this forces you to avoid the problems that can occur when
    // functors are copied by algorithms (see Josuttis, p486 and prior).
    bool operator()(int x) const
    {
        return x % 2 == 0;
    }
};

int main(int argc, char *argv[])
{
    vector<int> src { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };
    remove_copy_if(src.begin(), src.end(),
                   ostream_iterator<int>(cout, " "),
                   MyUnaryPred());
    cout << "\n\n";
    cout << "The pre-defined functors (Josuttis, p486).:\n"
        "These are all available in <functional>.\n\n"
        "Expression            Effect\n"
        "==========            ======\n"
        "negate<type>()        -param\n"
        "plus<type>()          param1 + param2\n"
        "minus<type>()         param1 - param2\n"
        "multiplies<type>()    param1 * param2\n"
        "divides<type>()       param1 / param2\n"
        "modulus<type>()       param1 % param2\n"
        "equal_to<type>()      param1 == param2\n"
        "not_equal_to<type>()  param1 != param2\n"
        "less<type>()          param1 < param2\n"
        "greater<type>()       param1 > param2\n"
        "less_equal<type>()    param1 <= param2\n"
        "greater_equal<type>() param1 >= param2\n"
        "logical_not<type>()   !param\n"
        "logical_and<type>()   param1 && param2\n"
        "logical_or<type>()    param1 || param2\n"
        "bit_and<type>()       param1 & param2\n"
        "bit_or<type>()        param1 | param2\n"
        "bit_xor<type>()       param1 ^ param2\n";

    return 0;
}
