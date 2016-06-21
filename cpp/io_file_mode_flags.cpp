#include <fstream>
#include <iostream>
#include <string>

using namespace std;

const string fname = "xyz.tmp";

void open_for_writing()
{
    ofstream f(fname, ios_base::out);
    f << "Some data\n";
}

void open_for_reading(const string& prefix)
{
    ifstream f(fname);
    string s;
    while (getline(f, s))
    {
        cout << prefix << ": " << s << "\n";
    }
}

void open_for_append()
{
    ofstream f(fname, ios_base::app);
    f << "Some *more* data\n";
}

void open_for_random_reading()
{
    ifstream f(fname);
    f.seekg(5);
    string s;
    f >> s;
    cout << "Got s = " << s << "\n";
}

void open_for_random_writing()
{
    // There is no way to insert new data in the middle of a file since it is
    // not supported by the OS; the solution is to write to a new file then
    // replace the old file when the new file is successfully written.

    // However, it is possible to REPLACE data in the middle of a file.
    ofstream f(fname, ios::in | ios::out);
    f.seekp(5, ios::beg);
    f.write("DATA", 4);
}

int main(int argc, char *argv[])
{
    // See Josuttis, p796 or
    // http://en.cppreference.com/w/cpp/io/ios_base/openmode

    // Flag              Meaning
    // ====              =======
    // in                Opens for reading (default for ifstream)
    // out               Opens for writing (default for ofstream)
    // app               Always appends at the end when writing
    // ate               Positions at the end of the file after opening ("at end")
    // trunc             Remove the former file contents
    // binary            Does not replace special characters

    // ios_base Flags    Meaning                                            C Mode
    // ==============    =======                                            ======
    // in                Reads (file must exist)                            "r"
    // out               Empties and writes (creates if necessary)          "w"
    // out|trunc         Empties and writes (creates if necessary)          "w"
    // out|app           Appends (creates if necessary)                     "a"
    // app               Appends (creates if necessary)                     "a"
    // in|out            Reads and writes; initial position is the start (file must exist)   "r+"
    // in|out|trunc      Empties, reads, and writes (creates if necessary)  "w+"
    // in|app            Updates at end (creates if necessary)              "a+"
    // in|out|app        Updates at end (creates if necessary)              "a+"

    // In normal text mode (when binary is not set), when running on Windows
    // then newline characters are replaced by the two character CR-LF sequence.

    open_for_writing();
    open_for_reading("First read");
    open_for_append();
    open_for_reading("Second read");
    open_for_random_reading();
    open_for_random_writing();
    open_for_reading("Third read");

    return 0;
}
