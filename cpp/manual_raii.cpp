#include <cstdio>
#include <string>

// A canonical example of manual RAII.
// The close() method allows early closing, but you don't have to use it.

class X
{
public:
  X()
  {
    fp = std::fopen("whatever", "r");
    if (fp == 0)
      throw "failed to open file";
  }

  ~X()
  {
    try
    {
      close();
    }
    catch (const std::string& s)
    {
      // Perhaps do something here.
    }
  }

  void close()
  {
    if (fp != 0)
    {
      if (fclose(fp) != 0)
      {
      }
      fp = 0;
    }
  }

private:
  FILE *fp = 0;
};

int main(int argc, char *argv[])
{

  return 0;
}
