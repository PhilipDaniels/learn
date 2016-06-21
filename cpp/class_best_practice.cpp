#include <iostream>
#include <string>
#include "class_best_practice.hpp"

using std::cout;
using std::endl;

// In the cpp file.
int Good::box() const { return 3; }

Good::Good()
  : Good("ctor delegation example")
{
}

Good::Good(std::string s)
  : data(s)
{
  num_instances++;
}

Good& Good::return_ref_to_this()
{
  cout << "I am chaining..." << endl;
  return *this;
}

const Good& Good::constr_return_ref_to_this() const
{
  cout << "I am const chaining..." << endl;
  return *this;
}

std::ostream& print(std::ostream& os, const Good& g)
{
  cout << "Printing, foo = " << g.foo() << ", bar = " << g.bar() << endl;
  return os;
}

// Note that we do not use the static keyword on the definition.
int Good::get_num_instances()
{
  return num_instances;
}
int Good::num_instances { 0 };


int Good::get_num_days()
{
  return num_days;
}
constexpr int Good::num_days;

///////////////////////////////////////////////////////////////////////////////
// Usage of the class.
int main(int argc, char *argv[])
{
  Good g;
  cout << "foo = " << g.foo() << ", bar = " << g.bar() << endl;
  print(cout, g);

  cout << g.return_ref_to_this().foo() << endl;
  Good g2;
  cout << "num_instances = " << Good::get_num_instances() << endl;
  Good g3;
  cout << "num_instances = " << Good::get_num_instances() << endl;
  cout << "num_days = " << Good::get_num_days() << endl;

  return 0;
}
