#include <algorithm>
#include <cassert>
#include <functional>
#include <string>
#include "string_utils.hpp"

using std::string;
using std::vector;

bool starts_with(const string& haystack,
                 const string& needle,
                 CharComparison cmp)
{
    if (needle.length() > haystack.length())
        return false;

    for (auto i = 0; i < needle.length(); ++i)
    {
        if (!cmp(needle[i], haystack[i]))
            return false;
    }

    return true;

}

bool ends_with(const string& haystack,
               const string& needle,
               CharComparison cmp)
{
     // Careful: auto is an unsigned long! A nasty bug in the loop.
    int nlen = needle.length();
    int hlen = haystack.length();

    if (nlen > hlen)
        return false;

    for (auto ni = nlen - 1, hi = hlen - 1; ni >= 0; --ni, --hi)
    {
        if (!cmp(needle[ni], haystack[hi]))
            return false;
    }

    return true;
}

string::const_iterator find(const string& haystack,
                            const string& needle,
                            CharComparison cmp)
{
    auto pos = std::search(haystack.begin(), haystack.end(),
                           needle.begin(), needle.end(),
                           cmp);
    return pos;
}

int main(int argc, char *argv[])
{
    assert(starts_with("hello", "he"));
    assert(starts_with("hello", "HE", icharcmp));
    assert(ends_with("hello", "lo"));
    assert(ends_with("hello", "LO", icharcmp));

    string s1("  hello  ");
    trim(s1);
    assert(s1 == "hello");

    string s2("  helloxx");
    trim(s2, "ox");
    assert(s2 == "  hell");

    string s3("  hello  ");
    string s4 = rtrim(ltrim(s3));
    assert(s4 == "hello");

    // How to call the split function.
    vector<string> elems;
    split("a,b,c", elems, ",");
    assert(elems[0] == "a");
    assert(elems[1] == "b");
    assert(elems[2] == "c");
    assert(elems.size() == 3);

    elems.clear();
    split("a,.b", elems, ",.");
    assert(elems[0] == "a");
    assert(elems[1] == "");
    assert(elems[2] == "b");

    elems.clear();
    split("a,.b", elems, ",.", true);
    assert(elems[0] == "a");
    assert(elems[1] == "b");

    string s5("Hello World");
    auto it = find(s5, "world");
    assert(it == s5.end());
    it = find(s5, "world", icharcmp);
    assert(*it == 'W');

    return 0;
}
