#include <iostream>
#include <string>

using std::cout;
using std::endl;

// initializer_list allows only 1 type.
void doit(std::initializer_list<int> params)
{
    for (auto &p : params)
    {
	cout << p << endl;
    }
}

void doit2(std::string msg, std::initializer_list<int> params)
{
    cout << msg << endl;

    for (auto &p : params)
    {
	cout << p << endl;
    }
}


int main(int argc, char *argv[])
{
    doit({2, 3, 4, 10, 20});
    doit2("hello", {98, 99, 100});
    return 0;
}
