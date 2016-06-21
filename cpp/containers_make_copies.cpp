#include <iostream>
#include <string>
#include <vector>

using namespace std;

int main(int argc, char *argv[])
{
    // Key concept: when you add an object to a container it MAKES A COPY.
    // If you want reference semantics, use a shared_ptr.
    string s("hello world");
    vector<string> v;
    v.push_back(s);
    s = "new string";

    cout << "s = \"" << s << "\" but v[0] = \"" << v[0] << "\"" << endl;

    // You can also CONSTRUCT elements using emplace_front, emplace and
    // emplace_back. The args to emplace* must match a constructor of the
    // element type.

    return 0;
}
