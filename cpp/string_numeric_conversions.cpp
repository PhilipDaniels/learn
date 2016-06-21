#include <iostream>
#include <string>

using namespace std;

int main(int argc, char *argv[])
{
    // Note, neither to_string or stod() etc does not work in Cygwin.
    // See http://stackoverflow.com/questions/12975341
    // See http://en.cppreference.com/w/cpp/string/basic_string/to_string
    // for equivalences.

    int i = 42;
    string s = to_string(i);
    wstring ws = to_wstring(i);

    string s2("3.14");
    double d = stod(s2);

    // We also have stoi, stol, stoul, stoll, stoull, stof, stod and stold.

    return 0;
}
