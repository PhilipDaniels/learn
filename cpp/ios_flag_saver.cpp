#include <iostream>
#include <ostream>
#include "ios_flag_saver.hpp"

using namespace std;

void print_with_restore(ostream& os)
{
    IosFlagSaver saver(os);
    os << hex << 24 << "\n";
}

void print_without_restoring(ostream& os)
{
    os << hex << 24 << "\n";
}

int main(int argc, char *argv[])
{
    // An example of how to use the IosFlagSaver.
    auto& os = cout;
    print_with_restore(os);
    os << 24 << "\n";

    os << "Now showing the problem if you don't restore:\n";
    print_without_restoring(os);
    os << 24 << "\n";

    return 0;
}
