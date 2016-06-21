#include <iostream>
#include <sstream>
#include "eatws.hpp"

using namespace std;

int main(int argc, char *argv[])
{
    istringstream ss("   hello world");
    cout << ss.rdbuf() << "\n";

    istringstream ss2("   hello world");
    ss2 >> eatws;
    // For demo only, to show we have advanced. rdbuf destroys the ptr.
    cout << ss2.rdbuf() << "\n";

    return 0;
}
