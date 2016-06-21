#include <initializer_list>
#include <memory>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

using std::initializer_list;
using std::make_shared;
using std::out_of_range;
using std::pair;
using std::shared_ptr;
using std::string;
using std::vector;


template <typename T>
class Blob
{
public:
    // This is just a way of making T available, e.g.
    //   "Blob<int>::value_type a = 3";
    // See also http://stackoverflow.com/questions/610245
    typedef T value_type;
    typedef typename vector<T>::size_type size_type;

    Blob();
    Blob(initializer_list<T> il);
    size_type size() const { return data->size(); }
    void push_back(const T& value) { data->push_back(value); }
    void push_back(T&& value) { data->push_back(std::move(value)); }
    void pop_back() { data->pop_back(); }
    T& back() { return data->back(); }
    T& operator[](size_type i)
    {
        check(i, "subscript out of range");
        return (*data)[i];
    }
    // This use of Blob, inside the class, is understood to be Blob<T> of the
    // appropriate type.
    void add(Blob new_values) { }

private:
    shared_ptr<vector<T>> data;
    void check(size_type i, const string& msg) const;
};


// To define a member outside the class, do this.
template <typename T>
void Blob<T>::check(size_type i, const string& msg) const
{
    if (i >= data->size())
        throw out_of_range(msg);
}

// Likewise for constructors.a
template <typename T>
Blob<T>::Blob()
    : data(make_shared<vector<T>>())
{
}


// Lippman, p666, template type aliases.
// We can make a typedef for a closed template type like this.
typedef Blob<int> IntBlob;
// But we need to use "using" for the open generic type alias.
template <typename T> using twin = pair<T, T>;
template <typename T> using twinWithInt = pair<T, int>;


int main(int argc, char *argv[])
{
    // n.b. Class member functions are only instantiated if used.
    // This helps to reduce code bloat.
    Blob<string> sblob;
    sblob.push_back("hello");

    // will throw.
    // string s = sblob[12];

    IntBlob ib;
    twin<string> authors;

    return 0;
}
