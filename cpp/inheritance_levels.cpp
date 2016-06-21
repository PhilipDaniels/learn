#include <iostream>

using std::cout;

class A
{
public: int x;
protected: int y;
private: int z;
};

class B : public A
{
    // x is public
    // y is protected
    // z is not accessible
};

class C : protected A
{
    // x is protected
    // y is protected
    // z is not accessible
};

class D : private A
{
    // x is private
    // y is private
    // z is not accessible
};

// Preventing inheritance.
class Z final { };

int main(int argc, char *argv[])
{
    cout << "Private inheritance is the default, but you should really\n"
         << "only use public inheritance because it expresses IS-A.\n";

    return 0;
}
