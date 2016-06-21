#include <algorithm>
#include <functional>
#include <iostream>
#include <string>
#include <vector>

using namespace std::placeholders;
using namespace std;

int sum(int x, int y, int z)
{
    cout << "x=" << x << " y=" << y << " z=" << z << endl;
    return x + y + z;
}


// This example from Josuttis, p481.
class Person
{
public:
    Person(const string& name) : name_(name) {}
    void print() const { cout << name_ << "\n"; }
    void print2(const string& prefix) { cout << prefix << name_ << "\n"; }
private:
    string name_;
};

int main(int argc, char *argv[])
{
    // Hardwire x to 10, y to 20 and pass the remaining arg as z.
    auto f = bind(sum, 10, 20, _1);
    cout << f(100) << endl;

    // Reorder parameters.
    auto f2 = bind(sum, _3, _2, _1);
    f2(40, 50, 60);

    // Call a member function.
    vector<Person> people { { "Tick" }, { "Trick" }, { "Track" } };
    for_each(people.begin(), people.end(), bind(&Person::print, _1));
    for_each(people.begin(), people.end(), bind(&Person::print2, _1, "Name: "));

    // But this is actually easier with mem_fn.
    for_each(people.begin(), people.end(), mem_fn(&Person::print));

    return 0;
}
