#include <iostream>
#include <string>

#ifndef CLASS_BEST_PRACTICE_HPP
#define CLASS_BEST_PRACTICE_HPP

// In the header file.
// * Do not use any "usings" in the header.
// * Every member must be declared in its class, we can define in or outside.

class Good
{
public:
    Good();

    // Explicit applies to ctors with 1 parameter and stops them from
    // being used in automatic type conversions.
    explicit Good(std::string s);

    // Implicitly inline functions are defined here.
    int foo() const { return 1; }
    // Explicit inline functions are done like this (both parts in the header).
    inline int bar() const;
    // Non-inline functions can be defined in the header or the cpp.
    int box() const;

    // How to return a ref to this. Note the difference for const members.
    Good& return_ref_to_this();
    const Good& constr_return_ref_to_this() const;

    static int get_num_instances();
    static int get_num_days();

private:
    std::string data;
    // This static variable is actually defined in the cpp file.
    static int num_instances;
    // Lippman, 7.6, p303. Still need a definition.
    static constexpr int num_days = 30;
    // Initialize variables like this rather than in constructors.
    bool valid = true;
};

// Here is the body for my explicitly inline function.
int Good::bar() const { return 2; }

// Non-member functions that are logically part of the interface are declared
// in the header but usually implemented in the cpp file.
std::ostream& print(std::ostream& os, const Good& g);


#endif /* CLASS_BEST_PRACTICE_HPP */
