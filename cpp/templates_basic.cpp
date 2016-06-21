#include <iostream>
#include <string>

using std::cout;
using std::string;

/*
 * Templates - both declarations and definitions - reside in headers.
 */


// A simple template with 1 type parameter.
// Can write "class" instead of "typename" - there is no difference.
template <typename T> /* can write inline or constexpr here */
int compare(const T& v1, const T& v2)
{
    if (v1 < v2)
        return -1;
    if (v2 < v1)
        return 1;
    return 0;
}

// Templates can have non-type parameters. They must be constant expressions.
template <int N>
int fn_with_nontype_parameter(string s)
{
    for (int i = 0; i < N; i++)
    {
        cout << "Hello, " << s << "\n";
    }
}

int main(int argc, char *argv[])
{
    if (compare(12, 20) <= 0)
        cout << "yep\n";
    else
        cout << "no\n";

    fn_with_nontype_parameter<3>("world");

    return 0;
}
