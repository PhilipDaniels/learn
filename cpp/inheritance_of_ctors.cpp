#include <string>

using std::string;

class Base
{
public:
    Base(string name)
        : name_(name), price_(10)
    { }

    Base(string name, double price)
        : name_(name), price_(price)
    { }

private:
    string name_;
    double price_;
};


class Sub : public Base
{
public:
    // This is how to inherit constructors.
    using Base::Base;
};


int main(int argc, char *argv[])
{
    Sub s1("sub1");
    Sub s2("sub2", 20.0);
    return 0;
}
