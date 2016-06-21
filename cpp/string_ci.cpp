#include <iostream>
#include <map>
#include "string_ci.hpp"

using namespace std;

int main(int argc, char *argv[])
{
    cistring s1("hallo");
    cistring s2("HALlo");

    cout << boolalpha;
    cout << s1 << " = " << s2 << " : " << (s1 == s2) << "\n";

    // We can now make a case-insensitive map.
    map<cistring, int> m;
    m[s1] = 20;
    m[s2] = 30;
    cout << "m[hallo] = " << m[s1] << "\n";

    return 0;
}
