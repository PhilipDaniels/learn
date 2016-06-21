#include <functional>
#include <iostream>
#include <set>
#include <string>

using namespace std;

class Thing
{
    friend bool operator<(const Thing& lhs, const Thing& rhs);
    friend bool operator>(const Thing& lhs, const Thing& rhs);

public:
    string name;
    int id;
};


bool operator<(const Thing& lhs, const Thing& rhs)
{
    return lhs.name < rhs.name;
}

bool operator>(const Thing& lhs, const Thing& rhs)
{
    return lhs.name > rhs.name;
}


int main(int argc, char *argv[])
{
    // By default, order things by name.
    set<Thing> thing_set1 { { "First", 200 }, { "Alpha", 100 } };

    for (const auto &t : thing_set1)
    {
        cout << t.name << "\n";
    }
    cout << "\n";


    // But we can create a set using a custom ordering predicate.
    set<Thing, greater<Thing>> thing_set2 { { "First", 200 }, { "Alpha", 100 } };

    for (const auto &t : thing_set2)
    {
        cout << t.name << "\n";
    }
    cout << "\n";


    // Or even by projection.
    auto byid = [] (const Thing& lhs, const Thing& rhs) { return lhs.id < rhs.id; };
    set<Thing, decltype(byid)> thing_set3(byid);
    thing_set3.insert({ { "First", 200 }, { "Alpha", 100 } });

    for (const auto &t : thing_set3)
    {
        cout << t.name << "\n";
    }


    return 0;
}
