#include <iostream>
#include <tuple>

using std::cout;
using std::get;
using std::make_tuple;
using std::tuple;

int main(int argc, char *argv[])
{
    tuple<int, int, int> point3d { 10, 20, 30 };

    auto item = make_tuple("hello", 2, 20.0);
    cout << "item = (" << get<0>(item) << "," << get<1>(item) << ","
         << get<2>(item) << ")\n";
    get<0>(point3d) *= 5;
    cout << "point3d(0) = " << get<0>(point3d) << "\n";

    return 0;
}
