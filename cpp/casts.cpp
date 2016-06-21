#include <iostream>

using std::cout;
using std::endl;

class MyBase
{
  public:
  virtual void test() { }
};
class MyChild : public MyBase { };


int main(int argc, char *argv[])
{
    // static_cast. Used where we know the types in advance, such as in
    // arithmetic conversions. Performs no runtime checking.
    double slope = static_cast<double>(7) / 18.0;


    // const_cast adds or removes a const qualification. The input can be const
    // or non-const.  They are often used with overloaded functions (Lippman,
    // p232). First some remove examples:
    const char *cpc;
    char *pc;
    char *p = const_cast<char*>(cpc);   // Ok, but writing through p is undefined.
    p = const_cast<char*>(pc);          // Ok, writing through p is defined.
    // Then an add-const example.
    const char *c = const_cast<const char*>(pc);


    // dynamic_cast is used to convert pointers and references at run time,
    // generally for the purpose of casting a pointer or reference up or down an
    // inheritance hierarchy. The target type must be a pointer or reference
    // type and nullptr is returned if the case fails (in other words, a runtime
    // check is performed, and in fact it has similar semantics to "as" in C#).
    // Also, the base class must have at least one virtual function.
    //
    // When references are used with dynamic_cast instead of pointers, a "null
    // reference" cannot be returned, so instead it throws a std::bad_cast
    // exception which you have to catch.
    MyChild *child = new MyChild();
    MyBase *base1 = child;                           // Ok because derived.
    MyBase *base2 = dynamic_cast<MyBase*>(child);    // Equivalent.
    // MyChild *child2 = base1;                      // Won't compile.
    MyChild *child2 = dynamic_cast<MyChild*>(base1); // Ok, checked at runtime.
    if (child2)
	cout << "Successfully converted base1 to child2" << endl;
    else
	cout << "Could not convert base1 to child2" << endl;


    // reinterpret_cast is a low-level conversion of one bit pattern to another.
    // They are machine dependent and should be avoided (Lippman, p164).

    cout << "\nAdvice: avoid casts, and never use c-style casts.\n"
	 << "static_cast, const_cast and dynamic_cast have occasional use cases,\n"
	 << "but reinterpret_cast should never be used unless you are writing\n"
	 << "low-level machine-dependent code." << endl;

    return 0;
}
