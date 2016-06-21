#include <algorithm>
#include <iostream>
#include <iterator>
#include <set>
#include <vector>

using namespace std;

void demo1()
{
    vector<string> words;

    // Grab all words from stdin. The () construct creates an "end of stream
    // iterator".
    copy(istream_iterator<string>(cin),
         istream_iterator<string>(),
         back_inserter(words));

    sort(words.begin(), words.end());

    cout << "\n\nWriting output...\n\n";

    // Print all words without duplicates.
    unique_copy(words.cbegin(), words.cend(),
                ostream_iterator<string>(cout, "\n"));
}

void demo2()
{
    set<string> words;

    // Grab all words from stdin.
    copy(istream_iterator<string>(cin),
         istream_iterator<string>(),
         inserter(words, words.begin()));

    // No need to sort as a set does that automatically.

    cout << "\n\nWriting output...\n\n";

    // Print all words without duplicates.
    unique_copy(words.cbegin(), words.cend(),
                ostream_iterator<string>(cout, "\n"));
}

int main(int argc, char *argv[])
{
    demo2();
    return 0;
}
