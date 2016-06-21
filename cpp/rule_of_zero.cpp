#include <iostream>
#include <memory>
#include <cstring>
#include "foo.hpp"

using std::cout;
using std::endl;

// Rule Of Zero
// ============
// * Do not use raw pointers to manage resources, use smart pointers.
//     - unique_ptr if a class can be moved, but not copied and does not have to be shared
//     - shared_ptr if a class can be copied and has to be shared
// * Do not use c-style constructs
//     - std::string instead of char*
//     - std::array instead of of c arrays, or std::vector (because c-style
//       arrays are just disguised pointers anyway).
//
// A class that deal with ownership should not do anything else (SRP).
//
// If you follow this then the compiler will create all the correct constructors
// and copy/move assignment operators for you - you should not write any of them!
//
// -----
// Stated another way, the rule of zero says:
//   You should NEVER implement a destructor, copy/move constructor or
//   copy/move assignment operators in your code.
// ASSUMING
//   You should NEVER use a raw pointer to manage a resource.
//
// -----
// An alternative phrasing is the Rule of All or Nothing from
// http://arne-mertz.de/2015/02/the-rule-of-zero-revisited-the-rule-of-all-or-nothing/
//
// "A class that needs to declare one or more of the destructor, copy/move
// constructor or copy/move assignment operations should explicitly default the
// rest of those operations. All other classes should not declare any
// destructor, copy/move constructor or copy/move assignment operator."


// Start here:
// https://rmf.io/cxx11/rule-of-zero/
// http://blog.florianwolters.de/educational/2015/01/31/The_Rule_of_Zero/
// https://blog.feabhas.com/2015/01/the-rule-of-zero/
// http://stackoverflow.com/questions/12184779/using-stdunique-ptr-for-windows-handles?lq=1
// http://stackoverflow.com/questions/14878121/is-there-a-proper-ownership-in-a-package-for-handles-available
//
// and in particular scope_exit and unique_resource which will be added to the
// standard. The PDF contains a reference implementation at the end:
// http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2014/n4189.pdf


// However, all this seems overblown to me. The example below seems
// to work fine, with no need to worry about "handle semantics".
// The Rule of Zero renders the examples in Lippman about custom deleters
// irrelevant.

/*
The example cdeom from https://rmf.io/cxx11/rule-of-zero/ for non-ptr resources:

class module
{
public:
    explicit module(std::wstring const& name)
	: handle { ::LoadLibrary(name.c_str()), &::FreeLibrary } {}

    // other module related functions go here

private:
    using module_handle = std::unique_ptr<void, decltype(&::FreeLibrary)>;
    module_handle handle;
};
*/

// Ok this uses "new", but we could have LoadLibrary here instead.
// We might call it "FontHandle" if we were managing an HFONT.
class FooHandleManager
{
public:
  explicit FooHandleManager(std::string handle_name)
    : h(new FooHandle(handle_name), deallocate_foo_handle)
  {
  }

private:
  std::unique_ptr<FooHandle, decltype(&::deallocate_foo_handle)> h;
};



// Exists to prove passing as an arg does not do a spurious destruction.
void do_something(FooHandleManager& fhm)
{
  cout << "In do_something\n";
}


int main(int argc, char *argv[])
{
  {
    FooHandleManager m1("h1");
    do_something(m1);

    // Will not compile, copying is banned because of use of unique_ptr.
    //auto m2 = m1;
    //auto m3(m1);

    // This will compile, unique ptrs can be moved to a new owner.
    auto m4 = std::move(m1);
  }

  return 0;
}
