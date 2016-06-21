#ifndef FOO_HPP
#define FOO_HPP

#include <iostream>
#include <string>

// This class has a standard destructor, so when used with shared_ptr or
// unique_ptr via new/delete it works as expected.
class Foo
{
public:
  Foo(std::string name)
    : name(name)
    {
      std::cout << "Foo '" << this->name << "' is being constructed.\n";
    }

  ~Foo()
    {
      std::cout << "Foo '" << name << "' is being destroyed.\n";
    }

private:
  std::string name;
};


// This class does not have a destructor (except for PoC), instead an API
// function must be used to deallocate the resource. Many OS handles work this
// way, also things like database connections etc. It is based on the example in
// Lippman, $$ 12.1.4, page 465. Note that this is NOT a manager class for a
// Handle, it is more like a Windows HANDLE.
class FooHandle
{
public:
  FooHandle(std::string name)
    : name(name)
  {
    std::cout << "FooHandle '" << this->name << "' is being constructed.\n";
  }

  std::string get_name() { return name; }

  ~FooHandle()
  {
    std::cout << "FooHandle '" << name << "' is being destroyed.\n";
  }

private:
  std::string name;
};

void deallocate_foo_handle(FooHandle* hf)
{
  std::cout << "FooHandle '" << hf->get_name() << "' is being de-allocated.\n";
}

#endif /* FOO_HPP */
