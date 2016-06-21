#include <functional>
#include <string>

using std::string;

// If you want a function to take a parameter which is a callable, you
// declare it like this.
//    std::function<TReturn(TArg1 arg1, TArg2 arg2, etc.)>
// It is often easier with a typedef:

typedef std::function<bool(char lhs, char rhs)> CharComp;

bool starts_with(const string& haystack, const string& needle,
                 CharComp cmp = [] (char l, char r) { return l == r; })
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

// We can now call starts_with with any type of "callable" - a lambda, a
// function pointer, or a function object.

bool icharcmp(char lhs, char rhs)
{
    return tolower(lhs) == tolower(rhs);
}

struct Comparer
{
    bool operator()(char lhs, char rhs) const { return lhs == rhs; }
};


int main(int argc, char *argv[])
{
    // Call it with a function pointer.
    starts_with("Hello", "he", icharcmp);

    // Call it with a lambda.
    starts_with("Hello", "he", [] (char lhs, char rhs) { return lhs == rhs; });

    // Call it with a functor.
    Comparer c;
    starts_with("Hello", "he", c);
}
