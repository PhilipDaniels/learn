#include <cstddef>
#include <unordered_map>
#include "hashval.hpp"

using namespace std;

struct Customer
{
    string first_name;
    string last_name;
    int id;
    size_t age;
};

size_t get_hash(const Customer& c)
{
    return hash_val(c.first_name, c.last_name, c.id, c.age);
}

struct CustomerHasher
{
    size_t operator()(const Customer& c) const
    {
        return hash_val(c.first_name, c.last_name, c.id, c.age);
    }
};



// A new type so that method 3 does not interfere with the other methods.
struct Customer2
{
    string first_name;
    string last_name;
    int id;
    size_t age;


};

bool operator==(const Customer2& lhs, const Customer2& rhs)
{
    return lhs.first_name == rhs.first_name &&
        lhs.last_name == rhs.last_name &&
        lhs.id == rhs.id &&
        lhs.age == rhs.age;
}

namespace std
{
    template <>
    struct hash<Customer2>
    {
        size_t operator()(const Customer2& c) const
        {
            return hash_val(c.first_name, c.last_name, c.id, c.age);
        }
    };
}

int main(int argc, char *argv[])
{
    // Method 1.
    // This will work, shows how to do it with a free function.
    // But the repetition is ugly.
    unordered_map<Customer, int, decltype(&get_hash)> um(100, get_hash);

    // Method 2.
    // If we create a hashing functor then we can get rid of the repetition.
    unordered_map<Customer, int, CustomerHasher> um2;

    // Method 3.
    // Create a specialization of std::hash for your type. This is the best way!
    // We will also need operator==.
    // Based on http://marknelson.us/2011/09/03/hash-functions-for-c-unordered-containers/
    unordered_map<Customer2, int> um3;
    Customer2 c { "Philip", "Daniels", 1, 500 };
    um3[c] = 42;

    return 0;
}
