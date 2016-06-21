#include <iostream>
#include <string>

using namespace std;

void throw_something()
{
    // Care! throw "a msg" throws a const char*.
    throw string("nasty msg");
}

// Used like this, noexcept means the function *claims* it won't throw.
void promise() noexcept
{
    // but it might be lying!
    throw "will compile, but will cause terminate to be called at runtime";

    // So noexcept means callers don't have to worry about handling exceptions,
    // because 1) the function really won't throw or 2) the program
    // unconditionally terminates.
}

// noexcept can take a bool argument. false means it can throw.
void promise2() noexcept(false)
{
}


void other(int i) { }

// The argument is often formed by using the noexcept operator, which
// leads to this brain damage. other(20) is not evaluated. This promises
// that promise3 will not throw if all of the functions specified in its
// noexcept(...) also promise not to throw (transitive closure).
void promise3() noexcept(noexcept(other(20)))
{
}

int main(int argc, char *argv[])
{
    cout << "Exception hierarchy (Lippman, p783, Josuttis p42)\n"
        "exception\n"
        "  bad_cast\n"
        "  bad_alloc\n"
        "    bad_array_new_length\n"
        "  bad_typeid\n"
        "  bad_exception\n"
        "  bad_weak_ptr\n"
        "  bad_function_call\n"
        "  runtime_error\n"
        "    overflow_error\n"
        "    underflow_error\n"
        "    range_error\n"
        "    system_error\n"
        "      ios_base::failure\n"
        "  logic_error\n"
        "    domain_error\n"
        "    invalid_argument\n"
        "    out_of_range\n"
        "    length_error\n"
        "    future_error\n\n\n";

    try
    {
        throw_something();
    }
    catch (string& msg)
    {
        // Catch something by reference allows us to mutate the thing.
        // We can also catch by value, we can still change it but only
        // the local copy is changed.
        cout << "Got a " << msg << "\n";
    }
    catch (...)
    {
        cout << "In the catch-all handler\n";
    }

    cout << "\nThere is a nasty issue with ctors throwing exceptions.\n"
        "To catch them, you need to write a 'function try block' -\n"
        "see Lippman p779.\n";

    promise();

    return 0;
}
