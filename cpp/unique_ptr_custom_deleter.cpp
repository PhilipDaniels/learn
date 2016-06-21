#include <iostream>
#include <memory>

using std::cout;
using std::unique_ptr;

////////////////////////////////////////////////////////////////////////////////
// Per Meyers, Effective C++, Item 18, it can be done this way.

// But don't do this - see rule_of_zero.cpp instead.

class Investment { };

auto delInvestment = [](Investment *pi)
{
  cout << "Deleting an investment.\n";
  delete pi;
};

// Factory function to create an investment.
unique_ptr<Investment, decltype(delInvestment)> makeInvestment()
{
  unique_ptr<Investment, decltype(delInvestment)> p { new Investment(), delInvestment };
  return p;
}

////////////////////////////////////////////////////////////////////////////////
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
////////////////////////////////////////////////////////////////////////////////

int main(int argc, char *argv[])
{
  {
    // We need to include the deleter's type in the template parameters:
    unique_ptr<Handle1, void(*)(Handle1*)> p1(new Handle1("unq handle"),
		       			    [] (Handle1* ot) { ot->Deallocate(); });
  }

  {
    auto p2 = makeInvestment();
  }

  return 0;
}
