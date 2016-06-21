#include <cstddef>
#include <string>

using std::size_t;
using std::string;


template <typename T> class Foo
{
public:
    static size_t count() { return ctr_; }
private:
    static size_t ctr_;
};


template <typename T>
size_t Foo<T>::ctr_ = 0;


int main(int argc, char *argv[])
{
    // Has its own ctr_.
    Foo<string> fs;
    // These 3 all share the same ctr_.
    Foo<int> fi1, fi2, fi3;

    auto s = Foo<string>::count();
    s = fs.count();
    
    return 0;
}
