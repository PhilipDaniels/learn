#include <iostream>
#include <map>
#include <set>
#include <string>

using std::cout;
using std::endl;
using std::make_pair;
using std::map;
using std::pair;
using std::set;
using std::string;

int main(int argc, char *argv[])
{
    // 3 dimensions, 8 containers.

    // Elements ordered by key (implemented with a red-black tree, O(log N))
    //   map, set
    //   multimap, multiset     - the key can appear multiple times

    // Unordered collections (implemented with a hash function, O(c))
    //   unordered_map, unordered_set
    //   unordered_multimap, unordered_multiset   - the key can occur multiple times

    // Meyer, Effective STL, item 23: A sorted vector may outperform the ordered
    // containers. It has no per-item overheard, whereas a tree has an overheard
    // of 3 pointers per item. This can lead to much better locality of
    // reference. Still, access patterns matter.


    map<string, size_t> counts;
    ++counts["hello"];
    ++counts["hello"];

    cout << "counts[\"hello\"] = " << counts["hello"] << "\n";


    // Associative container literals. Maps take a kvp.
    set<string> string_set = { "hello", "world" };
    map<string, int> string_int_map = { {"Jupiter", 14}, {"Earth", 1} };
    string_int_map.insert({ "Mars", 2 });

    for (const auto &kvp : string_int_map)
    {
        cout << kvp.first << " -> " << kvp.second << "\n";
    }
    string_int_map.erase("Mars");

    // To use a custom comparison function, see Lippman p425.


    // Pair is available if you need it.
    pair<string, bool> p { "True", true };
    auto p2 = make_pair(42, "the answer");


    // Algorithms are rarely used with the associative containers because the
    // keys are constant. And they have their own find() member. We only tend
    // to use them as sources or destinations (via "inserter").

    cout << endl;

    return 0;
}
