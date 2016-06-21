#include <iostream>

using std::cout;
using std::endl;

int main(int argc, char *argv[])
{
    auto x = 5;

    if (x < 10)
	goto some_label;
    cout << "I will never be seen" << endl;

some_label:
    cout << "Now at some_label (which must be in the same function as the goto)" << endl;

    return 0;
}
