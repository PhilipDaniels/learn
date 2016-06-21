#include <cstring>
#include <iostream>
#include <utility>

using std::cout;
using std::endl;

// Rule:

// If a class defines one of dtor, copy ctor, copy assignment operator then it
// should define all three. This typically means classes that manage raw
// pointers or handles. In practice, in C++11, that means five because you
// should also define the move constructor and assignment operator. It is not an
// error to omit the 'move' methods, just a missed optimization opportunity.

// See http://en.cppreference.com/w/cpp/language/rule_of_three

// BUT SEE THE RULE OF ZERO FOR A MUCH BETTER APPROACH.

class Manager
{
public:
    ~Manager() noexcept { delete [] data; }

    // Default constructor.
    Manager(const char* msg)
        : data(new char[std::strlen(msg) + 1])
    {
        std::strcpy(data, msg);
	cout << "Default ctor running" << endl;
    }

    // Copy constructor.
    Manager(const Manager& other)
	: data(new char[std::strlen(other.data) + 1])
    {
	std::strcpy(data, other.data);
	cout << "Copy constructor running" << endl;
    }

    // Copy assignment operator.
    Manager& operator=(const Manager& other)
    {
	char* tmp = new char[std::strlen(other.data) + 1];
	std::strcpy(tmp, other.data);
	delete[] data;
	data = tmp;
	cout << "Copy assignment running" << endl;
	return *this;
    }

    // Move constructor. (Data is moved from "other" into "this".
    Manager(Manager&& other)
	: data(other.data)
    {
	other.data = nullptr;
	cout << "Move constructor running" << endl;
    }

    // Move assignment operator.
    Manager& operator=(Manager&& other)
    {
	delete[] data;
        data = other.data;
	other.data = nullptr;
	cout << "Move assignment running" << endl;
	return *this;
    }

private:
    char* data;
};


int main(int argc, char *argv[])
{
    Manager m1("my message");
    Manager m2(m1);
    Manager m3 = m1;
    m3 = m1;
    m3 = std::move(m1);
    return 0;
}
