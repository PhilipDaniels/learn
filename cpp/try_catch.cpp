#include <iostream>
#include <stdexcept>
#include <string>

using std::cout;
using std::endl;

// See http://en.cppreference.com/w/cpp/error for further info

// Exception handling works similar to the way it does in C# - the first
// applicable handler is selected.

int main(int argc, char *argv[])
{
    try
    {
	throw std::runtime_error("I am an error.");
    }
    // You will get a compiler warning if you uncomment this "caught be earlier
    // handler".
    // catch (std::exception ex)
    // {
    // 	cout << "Caught a standard exception." << endl;
    // }
    catch (std::runtime_error ex)
    {
	cout << "Caught a runtime_error." << endl;
    }
    catch (std::exception ex)
    {
    	cout << "Caught a standard exception." << endl;
    }



    return 0;
}
