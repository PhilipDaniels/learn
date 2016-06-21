#include <functional>

using std::less;


// Default template arguments for a function.
template <typename T, typename F = less<T>>
int compare(const T& v1, const T& v2, F f = F())
{
    if (f(v1, v2)) return -1;
    if (f(v2, v1)) return 1;
    return 0;
}


// Default template arguments for a class.
template <class T = int>
class Numbers
{
public:
    Numbers(T v = 0)
        : val(v)
    { }
private:
    T val;
};


int main(int argc, char *argv[])
{
    Numbers<double> dn;
    Numbers<> in;
    return 0;
}
