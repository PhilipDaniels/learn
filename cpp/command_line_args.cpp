#include <iostream>

using std::cout;
using std::endl;

int main(int argc, char *argv[])
{
    // The arguments are as follows:

    // argv[0] = "name of myprog";  // or argv[0] might point to an empty string
    // argv[1] = "-d";
    // argv[2] = "-o";
    // argv[3] = "ofile";
    // argv[4] = "data0";
    // argv[5] = 0;

    cout << "argc is " << argc << endl;
    for (auto i = 0; i < argc; i++)
    {
	cout << "argv[" << i << "] = " << argv[i] << endl;
    }

    return 0;
}
