
int main(int argc, char *argv[])
{
    // This is from https://en.wikipedia.org/wiki/Anonymous_function

    // The general form of a lambda is:
    //   [capture](paramters) -> return_type { body };

    // Putting something in 'capture' makes the lambda into a closure.
    // The possibilities are:
    //   []        No variables defined. Attempting to use any external variables in the lambda is an error.
    //   [x, &y]   x is captured by value, y is captured by reference.
    //   [&]       Any external variable is implicitly captured by reference if used.
    //   [=]       Any external variable is implicitly captured by value if used.
    //   [&, x]    x is explicitly captured by value. Other variables will be captured by reference.
    //   [=, &z]   z is explicitly captured by reference. Other variables will be captured by value.

    // Variables captured by value are constant by default. Adding 'mutable'
    // after the parameter list makes them non-constant.
    // this can only be captured by value.

    auto lam = [](int x) { return x * 3; };
    auto lam_on_heap = new auto([](int x) { return x * 3; });

    int a = lam(2);
    int b = (*lam_on_heap)(3);

    // Lambdas can be used as hash functions, sorting functions, or equivalence
    // criterion, but functors are usually more concise and easier.
    // Josuttis, p504.

    return 0;
}
