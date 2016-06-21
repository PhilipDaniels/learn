#include <iostream>

using namespace std;

struct Point
{
    // Lippman, 7.5.5, p298.
    // All members must be public.
    // No ctors.
    // No in-class initializers.
    // No base classes or virtual functions.
    int x;
    int y;
};


int main(int argc, char *argv[])
{
    Point p1 = { 20, 40 };
    cout << "p1.x = " << p1.x << ", p1.y = " << p1.y << endl;

    cout << "Aggregate classes appear worse than simple DTOs because they\n"
        "are not even allowed to have constructors. They appear to be\n"
        "historical baggage and should be avoided. See also literal\n"
        "classes in Lippman, p300." << endl;

    return 0;
}
