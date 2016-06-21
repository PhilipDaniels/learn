#include <cstdlib>
#include <iostream>

using std::cin;
using std::cout;
using std::endl;

void enter_two_numbers()
{
    cout << "Enter two numbers:" << endl;
    int v1 = 0, v2 = 0;

    // This fills v1 and then v2.
    // Numbers must be separated by a space. "12,42" won't work.
    cin >> v1 >> v2;

    // endl flushes the buffer - needed for debugging statements!
    // But according to Stroustrup  https://github.com/isocpp/CppCoreGuidelines
    // should be avoided in everyday code, just use a \n instead and streams,
    // including cout, will be flushed when closed/destroyed.
    cout << "The sum of " << v1 << " and " << v2 << " is " << v1 + v2 << endl;
}

void enter_until_eof()
{
    cout << "Enter numbers then EOF (Ctrl-d on Unix, Ctrl-z on Windows):" << endl;
    int sum = 0, value = 0;
    while (cin >> value)
        sum += value;
    cout << "The sum is " << sum << endl;
}

void undefined()
{
    int n = 0;
    cout << "\nThe next line may print numbers in any order because there are no\n"
         << "sequence points on the line, so arguments can be evaluated in any order.\n";

    cout << n++ << " " << ++n << " " << n << " " << n++ << " " << n++ << "\n";
}

int main(int argc, char *argv[])
{
    enter_two_numbers();
    enter_until_eof();
    undefined();
    return EXIT_SUCCESS;
}
