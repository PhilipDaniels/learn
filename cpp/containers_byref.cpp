#include <iostream>
#include <map>
#include <string>
#include <utility>

using namespace std;

class Thing
{
public:
    Thing()
    {
        cout << "ctor (no args)\n";
    }

    Thing(const string& name)
        : name_(name)
    {
        cout << "ctor " << name_ << "\n";
    }

    ~Thing()
    {
        cout << "dtor " << name_ << "\n";
    }

private:
    string name_;
};


int main(int argc, char *argv[])
{
    map<string, Thing> m;
    m.emplace("key1", Thing("My thing"));
    cout << "The map is now constructed.\n";

    // I hoped that this would show the difference in behaviour,
    // but in fact there do not seem to be any calls to the ctor or dtor.
    auto t = m["key1"];
    cout << "Got t by value.\n";

    auto& u = m["key1"];
    cout << "Got u by reference.\n";

    return 0;
}
