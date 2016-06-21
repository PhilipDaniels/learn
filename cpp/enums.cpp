#include <cstdint>
#include <iostream>

using std::cout;
using std::uint8_t;

// In C++, enumerations are just integers.

// This creates a scoped enumeration.
enum class Color { Red, Yellow, Green };

// This creates an unscoped enumeration.
enum Animal { Ant, Beaver, Cat };

// Duplicate...
enum class Phosphors { Red, Yellow, Green };

// To specify the storage size. Can be signed or unsigned. The default is int.
enum class SmallEnum : uint8_t { Small, VerySmall };

int main(int argc, char *argv[])
{
    cout << (int)Color::Red << "\n";
    cout << (int)Beaver << "\n";

    Color x = Color::Red;
    // This won't compile, they are discrete types, whether or not
    // they are named enums.
    // x = Phosphors::Green;


    cout << "\nAdvice: always use scoped enums.\n";

    return 0;
}
