#include <iostream>

// This using declaration is visible from this point on.
// The scope can be reduced - it can appear inside a function, for example,
// or simply moved lower down in the file.
using std::cout;


/*
  In the .hpp, #include outside the namespce. Do not #include anything
  inside the namespace because that brings the names into the namespace.

  #include <....>

  namespace MySpace {
  }


  In the .cpp, again include before opening the namespace.

  #include "Mine.hpp"
  namespace MySpace {
  }

*/


// This is an unnamed namespace. Symbols declared in it are visible inside this
// file only. This replace the use of "file static" variables inherited from C.
namespace {
    int i;
}


// Name shortening.
namespace SomeVeryLongName { }
namespace sv = SomeVeryLongName;


// A using directtive. Import all names from SomeVeryLongName. Avoid.
// Only really safe in the implementation file for the namespace itself.
using namespace sv;


int main(int argc, char *argv[])
{
    i = 3;
    return 0;
}
