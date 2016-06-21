#include <cstddef>
#include <iostream>
#include <ostream>
#include <sstream>
#include <string>

using namespace std;

class SalesData
{
    friend ostream& operator<<(ostream& os, const SalesData& value);
    friend istream& operator>>(istream& is, SalesData& value);
    friend bool operator==(const SalesData& lhs, const SalesData& rhs);
    friend bool operator!=(const SalesData& lhs, const SalesData& rhs);
    friend bool operator<(const SalesData& lhs, const SalesData& rhs);

public:
    string isbn;
    double price;

    SalesData(const string& isbn, double price, size_t qty)
        : isbn(isbn), price(price), qty(qty) {}

    SalesData& operator+=(const SalesData& rhs);
    SalesData& operator=(string s);   // Assignment operator, acts as a type converter.
    string& operator[](size_t n);
    const string& operator[](size_t n) const;
    SalesData& operator++();     // Prefix.
    SalesData operator++(int);   // Postfix.

private:
    size_t qty;
};

// Pattern: takes a ref to os (because it gets mutated it cannot be const)
// and a ref to a const class type. Does minimal formatting.
// Declared as a friend so it can access private members.
//
// See Josuttis p810.
// These simple << and >> operators have two problems. They should be templates
// so they work with types other than char, and the operator<< does not handle
// field widths correctly. See io_template_insert_and_extract.cpp for a better
// example.
ostream& operator<<(ostream& os, const SalesData& value)
{
    os << value.isbn << " " << value.price << " " << value.qty;
    return os;
}

// Pattern: takes a ref to a non-const object because we will be changing it.
istream& operator>>(istream& is, SalesData& value)
{
    is >> value.isbn >> value.price >> value.qty;
    // After doing all input, check the stream using "if (is) ..."
    // If there was an error you might need to take action such as defaulting
    // value using the default constructor.

    // In addition, if we want to indicate failure we can set the failbit.
    // Do not change any other bits. But this seems rare, there are few
    // Google hits on this flag. Most people prefer exceptions?
    // See Lippman p559.
    if (value.isbn.length() < 3)
        is.setstate(is.rdstate() | ios::failbit);

    return is;
}

// Pattern: lhs and rhs are ref to const. A new object must be created and
// returned, this is usually done in a local variable. Implement in terms
// of the compound assignment operator.
SalesData operator+(const SalesData& lhs, const SalesData& rhs)
{
    SalesData sum = lhs;
    sum += rhs;
    return sum;
}

// Pattern: Define as a member. Return a ref to the lhs·
SalesData& SalesData::operator+=(const SalesData& rhs)
{
    qty += rhs.qty;
    return *this;
}

// Pattern: Non-member function, take refs to const, compare all members.
// If defining, also define !=.
bool operator==(const SalesData& lhs, const SalesData& rhs)
{
    return lhs.isbn == rhs.isbn &&
        lhs.price == rhs.price &&
        lhs.qty == rhs.qty;
}

bool operator!=(const SalesData& lhs, const SalesData& rhs)
{
    return !(lhs == rhs);
}

bool operator<(const SalesData& lhs, const SalesData& rhs)
{
    // A poor implementation.
    // Normally this operator should establish a total order.
    // See https://en.wikipedia.org/wiki/Total_order
    // and the good explanation at http://cpptips.com/stl_set_ordering
    // which says that STL containers require a total order.
    //
    // Where OP denotes some ordering function, for example "less than":
    // ≤
    // Anti-symmetric: if a OP b is true then b OP a is false.
    // Transitivity  : if a OP b and b OP c then a OP c
    // Irreflexivity : if a == b then a OP b is false
    //    (The above are sufficient for a partial order, some pairs of keys
    //     may be incomparable in that neither comes before the other.)
    // Totality      : a OP b or b OP a
    //    (The addition of totality means we can lay everything out in a
    //     line, hence a total order is also known as a linear order.)

    // See Josuttis, p315, for a definition of a strict weak order.

    return lhs.isbn < rhs.isbn &&
                      lhs.qty < rhs.qty;
}

// Pattern: return a reference to the lhs for consistency with the built-in
// operators and copy- and move-assignment.
SalesData& SalesData::operator=(string s)
{
    isbn = s;
    price = 0;
    qty = 0;
    return *this;
}

// Pattern: return a ref to the subscripted element and create const and
// non-const overloads of the operator.
string& SalesData::operator[](size_t n)
{
    return isbn;
}

const string& SalesData::operator[](size_t n) const
{
    return isbn;
}

// Pattern: define both prefix and postfix. Should normally define
// the full set of 4 operators.
SalesData& SalesData::operator++()   // Prefix.
{
    qty++;
    return *this;
}

SalesData SalesData::operator++(int)  // Postfix.
{
    // We must return the object as it exists before we mutate it.
    // In this case, we can only do that by making a copy.
    SalesData pre { this->isbn, this->price, this->qty };
    qty++;
    return pre;
}



int main(int argc, char *argv[])
{
    cout << "MEMBER?    OPERATOR\n"
         << "=======    ========\n"
         << "YES        assignment           : =\n"
         << "           subscript            : []\n"
         << "           function call        : ()\n"
         << "           member access        : ->\n"
         << "ADVISE-YES compound assignment  : +=, -=, *=, |= etc.\n"
         << "           increment, decrement : ++ --\n"
         << "           dereference          : *\n"
         << "ADVISE-NO  binary operators which can convert either operand\n"
         << "           arithmetic ops       : +, -, * / %.\n"
         << "           (in)equality         : == !=\n"
         << "           rel ops              : < > <= >=\n"
         << "           bit ops              : & | ^\n"
         << "NO         IO operators         : << >>\n";

    /*
     * Conversion operators are of the form "operator type() const". Avoid them,
     * they create confusing conversions. It is probably best to avoid assignment
     * operators for the same reason. The only exception is "operator bool" which
     * has use in conditions, and should be declared explicit.
     */

    SalesData sd { "myisbn", 40.0, 10 };
    cout << sd << "\n";

    cout << "Enter a SalesData object: ";
    cin >> sd;
    cout << "You entered " << sd << "\n";

    sd = "a new object via the assignment operator";
    cout << sd << "\n";

    // Note this must be on separate lines or behaviour is undefined due
    // to lack of sequence points. See io_basic.cpp.
    SalesData sd2 { "newbook", 100, 20 };
    cout << "sd2 starts as " << sd2 << "\n";
    cout << sd2++ << "\n";
    cout << ++sd2 << "\n";
    cout << sd2 << "\n";

    return 0;
}
