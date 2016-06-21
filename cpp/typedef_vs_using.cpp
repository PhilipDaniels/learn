#include <iostream>
#include <vector>

using std::cout;

typedef double wages;
typedef wages *dptr;
using wages = double;

using cwages = const double;

int main(int argc, char *argv[])
{
    using myset = std::vector<int>;

    double x = 44;
    dptr y = &x;

    cwages w2 = 22;
    //w2 = 23;        // Won't compile, w2 is a "const double".

    // const modifies the base type, so z is a constant pointer to char.
    // i.e. lowering typedefs is not done by simple textual substitution, it
    // is not "const double *z".
    const dptr z = &x;
    *z = 3;

    cout << "Guidance:\n"
	 << "  * Favour using over typedef. Using is a superset.\n";
}
