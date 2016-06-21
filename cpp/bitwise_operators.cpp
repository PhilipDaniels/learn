#include <bitset>
#include <iomanip>
#include <iostream>

using std::cout;
using std::endl;

int main(int argc, char *argv[])
{
    // PROMOTION FIRST
    // Operands are *FIRST*first promoted to integer size then any further
    // operations are carried out. This means that "~bits", if bits is an
    // unsigned char, means that all the bits in the high 3 bytes will be 1.

    // Bitwise operations on signed integers are machine-dependent.
    // Conclusion: always use unsigned int as the type.
    // The points noted below apply only to unsigned types.
    // Different rules may apply for signed types, see Lippman for details.

    // We define an int as follows

    // MSB                             LSB
    // Bit 31                        Bit 0
    // 00000000 00000000 00000000 00000000
    // 00000000 00000000 00000000 00000001  has value 1, or 2^^0
    // 00000000 00000000 00000000 00000010  has value 2, or 2^^1
    // 10000000 00000000 00000000 00000000  has value 2^^31, or 2,147,483,648
    // 11111111 11111111 11111111 11111111  has value 2^^32-1, or 4,294,967,295

    // Printing in binary can be most easily achieved using std::bitset.
    // Printing in hex is a disaster (see below).

    // << and >> shift in zeros.

    ////////////////////////////////////////////////////////////////////////////
    // Set a bit.
    auto x = 1u << 0;   // Set bit 0 and nothing else (value = 2^^0, or 1).
    cout << "x = " << x << endl;
    x = 1u << 4;        // Set bit 4 and nothing else (value = 2^^4, or 16).
    cout << "x = " << x << endl;

    x |= 1u << 3;       // Turn on bit 3 as well (value = 24).
    cout << "x = " << x << ", in binary = " << std::bitset<8>(x) << endl;
    cout << "in hex = 0x" << std::setfill('0') << std::setw(8) << std::right << std::hex << x << endl;

    // Unset a bit. This works by creating a value with just bit 4 turned on,
    // then inverts it, then bitwise ands it with x.
    x &= ~(1u << 4);    // Turn off bit 4 (value = 8).
    cout << "x = " << x << endl;

    // Test a bit.
    bool is_set = x & (1u << 8);
    cout << "Bit 8 is set = " << is_set << endl;
    is_set = x & (1u << 3);
    cout << "Bit 3 is set = " << is_set << endl;

    return 0;
}
