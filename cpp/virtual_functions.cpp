#include <iostream>
#include <string>

using std::cout;
using std::string;

class Base
{
public:
    Base(string name) : name_(name) { };
    virtual void print() { cout << name_; }
protected:
    string name_;
};


class Sub : public Base
{
public:
    Sub(string name) : Base(name) {}
    // Function signatures must match exactly. Can use "override" but this is
    // optional. "final" stops further overriding. Override is good because it
    // stops you making mistakes where you get the param list slightly wrong.
    void print() override { cout << "Base.Print = " << name_; };
};


class ABC
{
public:
    // Pure virtual function makes this an abstract base class.
    void virtual foo() = 0;
};

int main(int argc, char *argv[])
{
    Sub s1("sub1");
    s1.print();

    // Virtual functions that use default arguments should use the same defaults
    // as the function they override (see Lippman p607) it's because the default
    // used is the one from the static type, which can be confusing if they are
    // different.

    // Friendship is not inherited.
    return 0;
}
