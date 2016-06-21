#include <fstream>
#include <string>

using namespace std;

// We are returning a stack-allocated object here, but this works because in
// C++11 file streams have move semantics - as do string streams.
ofstream open_file(const string& filename)
{
    ofstream file(filename);
    return file;
}

int main(int argc, char *argv[])
{
    ofstream f;
    f = open_file("xyz.tmp");
    f << "Hello, world\n";

    return 0;
}
