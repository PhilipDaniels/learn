#include <algorithm>
#include <functional>
#include <iostream>
#include <string>

using std::cout;
using std::endl;
using std::string;

void initialization()
{
    string s1;                 // Empty string.
    string s2 = s1;
    string s3 = "hello";
    string s4(10, 'a');        // Repeat n times.
    string s5(s4, 1, 3);       // Use a substring.
    string s6("world");        // The trailing null is not included.

    cout << "s1 = " << s1
         << "\ns2 = " << s2
         << "\ns3 = " << s3
         << "\ns4 = " << s4
         << "\ns5 = " << s5
         << "\ns6 = " << s6 << endl;
}

void basics()
{
    string s1("hello");
    if (!s1.empty())
    {
        cout << "s1 is not empty. size() is " << s1.size() << endl;
    }
    s1 = "";
    if (!s1.empty())
    {
        cout << "s1 is not empty." << endl;
    }

    s1 = "hello";
    string s2("Hello");
    if (s1 < s2)
    {
        cout << s1 << "is less than " << s2 << endl;
    }
    else
    {
        cout << s1 << "is >= than " << s2 << endl;
    }

    // Concatenation is easy.
    s1 += " world";
}

void operations()
{
    cout << "\n\nOperations" << endl;
    string s1("hello");
    s1[0] = 'H';
    cout << "s1 = " << s1 << endl;

    s1.insert(0, "cruel ");
    cout << "s1 = " << s1 << endl;
    s1.erase(1, 3);
    cout << "s1 = " << s1 << endl;
}

void case_insensitive_find()
{
    // Surprisingly, there is no built in way of doing this kind of thing.
    // See libicu from IBM, Boost string algorithms
    using std::search;

    string text("foo BaR");
    string tofind("bar");

    // Using a local lambda.
    auto cmp = [] (char l, char r) -> bool { tolower(l) == tolower(r); };

    auto fpos = search(text.begin(), text.end(), tofind.begin(), tofind.end(), cmp);
    if (fpos != text.end())
        cout << "found at: " << distance(text.begin(), fpos) << endl;
}

void c_style()
{
    string s1("hello");
    // Get a c-style pointer to a null string.
    // The pointer will be invalidated if you call non-const string methods.
    // You do not need to free the pointer.
    const char *sptr = s1.c_str();
}

int main(int argc, char* argv[])
{
    // See http://en.cppreference.com/w/cpp/string/basic_string
    initialization();
    basics();
    operations();
    case_insensitive_find();
    c_style();

    // io. parsing lines. printf format strings.
    // read by lines.
}
