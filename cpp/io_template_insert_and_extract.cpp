#include <iomanip>
#include <ios>
#include <iostream>
#include <istream>
#include <ostream>
#include <sstream>

using namespace std;

// This is a canonical example of implementing operator>> and operator<<. The
// only thing it lacks is dynamic despatch - if your class will be inherited and
// you want << to work correctly under polymorphism, then you need to have
// operator<< delegate all the actual work to a virtual member function.
// See Josuttis, p815.

class Fraction
{
public:
    template <typename charT, typename traits>
    friend basic_ostream<charT, traits>& operator<<(basic_ostream<charT, traits>& strm,
                                                    const Fraction& value);

    template <typename charT, typename traits>
    friend basic_istream<charT, traits>& operator>>(basic_istream<charT, traits>& strm,
                                                    Fraction& value);

    Fraction(int num, int denom)
        : num_(num), denom_(denom)
    {
    }

private:
    int num_;
    int denom_;
};


template <typename charT, typename traits>
basic_ostream<charT, traits>& operator<<(basic_ostream<charT, traits>& strm,
                                         const Fraction& value)
{
    basic_ostringstream<charT, traits> s;
    s.copyfmt(strm);
    s.width(0);
    s << value.num_ << '/' << value.denom_;
    strm << s.str();
    return strm;
}

template <typename charT, typename traits>
basic_istream<charT, traits>& operator>>(basic_istream<charT, traits>& strm,
                                         Fraction& value)
{
    int n, d { 1 };
    strm >> n;

    while (strm.peek() == ' ')
        strm.ignore();

    if (strm.peek() == '/')
    {
        strm.ignore();
        strm >> d;
    }

    // If user enters 0 for denominator, signal an error.
    if (d == 0)
    {
        strm.setstate(ios::failbit);
    }
    else if (strm)
    {
        value = Fraction(n, d);
    }

    return strm;
}

int main(int argc, char *argv[])
{
    Fraction f(12, 27);
    cout << '*' << f << "*\n";
    cout << '*' << setw(10) << f << "*\n";

    cout << "Enter a fraction:\n";
    cin >> f;
    if (cin)
    {
        cout << "You entered " << f << "\n";
    }

    return 0;
}
