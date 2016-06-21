#include <iostream>

using std::cout;
using std::endl;

int main(int argc, char *argv[])
{
    // integer literals.
    auto a = 2;      // Decimal. Signed. smallest of int, long or long long in which it fits.
    auto b = 024;    // Octal. Signed or unsigned.
    auto c = 0x20;   // Hex (also 0X...). Signed or unsigned.

    // floating point literals.
    auto d1 = 3.14159;  // Double.
    auto d2 = 0.;
    auto d3 = 3e9;      // Also E is acceptable.

    // char and string literals.
    auto e = 'a';              // char
    auto f = "hello world\n";  // array of constant chars. Includes trailing null.

    cout << "adjacent " "strings " "are "
	"concatenated" << endl;

    cout << "some long string that goes \
on two lines but is really just one line" << endl;

    // Escape sequences.
    auto newline = '\n';
    auto carriage_return = '\r';
    auto horizontal_tab = '\t';
    auto backslash = '\\';
    auto double_quote = '\"';
    auto single_quote = '\'';
    auto vertical_tab = '\v';
    auto backspace = '\b';
    auto question_mark = '\?';
    auto formfeed = '\f';
    auto alert_aka_bell = '\a';
    auto esc1 = '\x21';          // hex number.
    auto esc2 = '\0';            // octal - 1 to 3 digits. This is the null char.
    auto esc3 = "\1234";         // more than 3 octal digits means this is 2 chars - \123 and '4'.

    // Prefixes - override the default type of a char or string literal.
    auto p1 = u'a';   // Unicode 16 - char16_t.
    auto p2 = U'a';   // Unicode 32 - char32_t.
    auto p3 = L'a';   // wide char - wchar_t.
    auto p4 = u8"a";  // utf-8 (applicable to string literals only) - char.

    // Suffixes - override the default type of an integer or floating point literal.
    auto s1 = 12u;    // unsigned, also U.
    auto s2 = 12L;    // long, also lowercase ell, but don't use that.
    auto s3 = 12LL;   // long long
    auto s4 = 12ULL;  // can combine - unsigned long long.
    auto s5 = 12.0f;  // float. Can't do "12f".
    auto s6 = 12.0L;  // long double.

    // Misc.
    auto b1 = false;   // Boolean. Also 'true'.
    auto n1 = nullptr; // null pointer literal.
}
