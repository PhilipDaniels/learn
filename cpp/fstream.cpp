#include <cstdio>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

using std::cerr;
using std::cout;
using std::endl;
using std::ifstream;
using std::istringstream;
using std::ofstream;
using std::string;
using std::vector;

void example1()
{
    const string filename("data.tmp");

    ofstream f(filename);
    if (!f)
    {
        cerr << "Could not open " << filename << endl;
        return;
    }

    f << "1 hello\n2 world\n3 again"<< endl;
    f.close(); // dtor will also close.

    ifstream fin(filename);
    string line;
    while (getline(fin, line))
    {
        cout << "Read line: " << line << endl;
    }


    // Delete the file.
    std::remove(filename.c_str());
}

// Example from Lippman, 8.3.1, p321.
struct PersonInfo
{
    string name;
    vector<string> phones;
};

void example2()
{
    string line, word;
    vector<PersonInfo> people;

    ifstream fin("fstream_person_info.txt");
    while (getline(fin, line))
    {
        PersonInfo pi;
        istringstream record(line);
        record >> pi.name;
        while (record >> word)
        {
            pi.phones.push_back(word);
        }
        people.push_back(pi);
    }

    for (const auto& p : people)
    {
        cout << "Got person = " << p.name << " with phones: ";
        for (const auto& phone : p.phones)
        {
            cout << phone << " ";
        }
        cout << endl;
    }
}

int main(int argc, char *argv[])
{
    example1();
    example2();
    return 0;
}
