#include <iostream>

using std::cout;
using std::endl;

int main(int argc, char *argv[])
{
    int x = 50;

    // nasty parentheses required to get this to compile.
    cout << (x < 10 ? "x is less than 10" : "x is more than 10");
    cout << endl;

    // When iostreams are not involved, it can be simpler.
    auto result = x < 10 ? "x is less than 10" : "x is more than 10";
    cout << result << endl;

    // Conditionals can be chained like this. Think: "Picks the true value if
    // the line matches, else goes on to the next line."
    result = x < 10 ? "x is less than 10" :
    	     x < 40 ? "x is less than 40" :
   	     x > 100 ? "x is more than 100" : "x is something else";

    cout << result << endl;

    return 0;
}
