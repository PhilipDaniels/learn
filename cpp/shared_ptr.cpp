#include <iostream>
#include <memory>
#include "foo.hpp"

using std::cout;
using std::endl;
using std::shared_ptr;
using std::string;
using std::make_shared;

//                copy?     move?
// shared_ptr     yes       yes
// unique_ptr     no        yes

// You should follow the rule_of_zero.cpp in your use of smart pointers.

int main(int argc, char *argv[])
{
  // We can use make_shared to create a shared_ptr that points
  // to an int with a value of 42. Generally, it takes ctor args.
  // make_shared is preferred to direct use of new (Meyers, Item 21).
  auto s1 = make_shared<int>(42);
  auto s2 = make_shared<string>(10, 'a');
  cout << "*s1 = " << *s1 << "\n*s2 = " << *s2 << "\n";

  shared_ptr<string> s3(new string("Example of direct use of new."));

  // Copying a ptr increments the reference count.
  auto f1 = make_shared<Foo>("Fred");
  auto f2 = f1;
  cout << "f1.use_count() = " << f1.use_count() << "\n";

  {
    // Automatically destroyed on exiting scope.
    auto f3 = make_shared<Foo>("Barney");
  }

  cout << "End of program.\n";
  return 0;
}
