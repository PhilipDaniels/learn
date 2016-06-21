#include <iostream>

using namespace std;

int main(int argc, char *argv[])
{
    cout << "Requirements of container elements (Josuttis, p244)\n"
        "===================================================\n"
        "1. Element must be copyable or movable.\n"
        "2. Element must be (move) assignable by the assignment operator.\n"
        "   Because containers and algorithms use assignment to overwrite elements.\n"
        "3. Element must be destructible.\n"
        "   Because containers destory elements when they remove them.\n\n"
        "In addition, an element MAY have to be\n"
        "======================================\n"
        "4. Default constructible.\n"
        "   Because some operations (e.g. growing) need to construct elements to fill slots.\n"
        "5. operator==. Needed for searching.\n"
        "   However, unordered containers allow you to provide your own equivalence definition.\n"
        "6. For associative containers, the sorting criterion.\n"
        "   This is usually operator<, which is called by less<>.\n"
        "   Bit if you use greater<>, you need to provide operator>, etc.\n"
        "7. For unordered containers, a hash function and an equivalence criterion.\n";

    return 0;
}
