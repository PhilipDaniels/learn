#include <iostream>

using std::cout;

int foo() { return 2; }
constexpr int bar() { return 2; }

int main(int argc, char *argv[])
{
    // constexpr must be something the compiler can evaluate at compile time.
    constexpr int a = 42;
    
    //constexpr int b = foo();     // Won't compile.
    const int b = foo();

    constexpr int c = bar();       // This is ok.
    constexpr auto d = bar();

    // constexpr is a top-level const - you cannot think of it as simply a
    // "better const".
    const int *p = nullptr;        // Pointer to const.
    constexpr int *q = nullptr;    // Const pointer (more akin to int *constexpr q);

    
    cout << "Guidance:\n"
         << "  * Favour constexpr over plain const.\n"
	 << "  * Remember that constexpr imposes a top-level const (Lippman, p67, $2.4.4).\n";

}
