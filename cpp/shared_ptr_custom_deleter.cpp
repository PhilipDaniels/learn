#include <iostream>
#include <memory>

using std::cout;
using std::shared_ptr;

// This example is taken directly from http://www.martyndavis.com/?p=474
// See http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2014/n4189.pdf
// for a standards proposal for a new class called unique_resource that
// is easier to use than unique_ptr.

// HOWEVER don't do this - use rule_of_zero.cpp instead.
class Handle1
{
 public:
  Handle1(std::string name)
    : name(name)
    {
      std::cout << "Handle1: " << this->name << " is being constructed.\n";
      was_deallocated = false;
    }

  void Deallocate()
  {
    was_deallocated = true;
    std::cout << "Handle1: " << name << " - Deallocate was called.\n";
  }

  ~Handle1()
  {
    if (was_deallocated)
    {
      std::cout << "Handle1: " << name << " is being destroyed, was correctly deallocated.\n";
    }
    else
    {
      std::cout << "Handle1: MEMORY LEAK! " << name
		<< " is being destroyed without being deallocated first.\n";
    }
  }

private:
  std::string name;
  bool was_deallocated;
};



int main(int argc, char *argv[])
{
  {
    Handle1 h1("Leaky");
    // oldThing going out of scope causes a "memory leak"
  }


  {
    // So let's use a smart pointer with a custom "deleter".
    // (Unfortunately there is no make_shared that takes a custom deleter).
    // Note the dtor does not get called!
    shared_ptr<Handle1> p1(new Handle1("tight"),
      [] (Handle1* th) {
        th->Deallocate();
      }
    );
  }

  {
  }

  return 0;
}
