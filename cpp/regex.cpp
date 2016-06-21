#include <iostream>
#include <regex>
#include <string>

using namespace std::regex_constants;
using namespace std;

void searching()
{
    // See Lippman 17.3.
    regex r("[[:alpha:]]*[^c]ei[[:alpha:]]*", regex::icase);
    smatch results;
    string input = "receipt frEInd theif receive";

    // This will find the first match.
    if (regex_search(input, results, r))
        cout << results.str() << "\n";

    // This will find all matches. end_it is a sregex_iterator that is empty.
    cout << "Now for all matches:\n";
    for (sregex_iterator it(input.begin(), input.end(), r), end_it; it != end_it; ++it)
    {
        cout << it->str() << "\n";
    }

    // Input sequence type  Regex classes (Lippman, p733).
    // ===================  ==============================================
    // string               regex,  smatch,  ssub_match,  sregex_iterator
    // const char*          regex,  cmatch,  csub_match,  cregex_iterator
    // wstring              wregex, wsmatch, wssub_match, wsregex_iterator
    // const wchar_t*       wregex, wcmatch, wcsub_match, wcregex_iterator

    // The match object lets you see the context of the match.
    // prefix, suffix etc.
    // Lippman, p736.
}

void out(bool value)
{
    cout << (value ? "found" : "not found") << "\n";
}

void checking_for_a_match()
{
    // After Josuttis, p718.
    // regex_match checks whether the entire input sequence matches.
    // regex_search checks whether a part of the input sequence matches -
    // regex_search(str, regex(pattern)) is equivalent to
    // regex_match(str, regex("(.|\n)*" + pattern + "*(.|\n)*"))
    cout << "\n\n checking_for_a_match() begins\n";

    regex reg1("<.*>.*</.*>");
    bool found = regex_match("<tag>value</tag>", reg1);
    out(found);

    // Here we create a group with the first () and then demand that
    // it is repeated with \1 (which needs escaping.)
    regex reg2("<(.*)>.*</\\1>");
    found = regex_match("<tag>value</tag>", reg2);
    out(found);

    // To avoid lots of slashes, we can use a raw string. Syntax is R"(....)",
    // or R"delim(...)delim" if you want to include )" inside the string.
    regex reg3(R"(<(.*)>.*</\1>)");
    found = regex_match("<tag>value</tag>", reg3);
    out(found);

    regex reg4("<(.*)>.*</\\1>");
    found = regex_match("extra stuff<tag>value</tag>", reg4);
    out(found);

    cout << "\nchecking_for_a_match() ends\n";
}

void print_match_details(bool found, const smatch& m)
{
    cout << "m.empty(): " << boolalpha << m.empty() << endl;
    cout << "m.size(): " << m.size() << endl;

    if (found)
    {
        cout << "m.str(): " << m.str() << endl;
        cout << "m.length(): " << m.length() << endl;
        cout << "m.position(): " << m.position() << endl;
        cout << "m.prefix().str(): " << m.prefix().str() << endl;
        cout << "m.suffix().str(): " << m.suffix().str() << endl;
        cout << endl;

        // iterating over all matches (using the match index):
        for (int i=0; i<m.size(); ++i)
        {
            cout << "m[" << i << "].str(): " << m[i].str() << endl;
            cout << "m.str(" << i << "): " << m.str(i) << endl;
            cout << "m.position(" << i << "): " << m.position(i) << endl;
        }
        cout << endl;

        // iterating over all matches (using iterators):
        cout << "matches:" << endl;
        for (auto pos = m.begin(); pos != m.end(); ++pos)
        {
            cout << " " << *pos << " ";
            cout << "(length: " << pos->length() << ")" << endl;
        }
    }
}

void using_the_match_results()
{
    cout << "\nusing_the_match_results() starts:\n";

    // After Josuttis, p720.
    string data = "XML tag: <tag-name>the value</tag-name>.";
    cout << "data: " << data << "\n\n";

    // m[0] will be all the matched data.
    // m[n] will be the captured groups.
    // m.prefix is the part before the match, and m.suffix is the part after.
    smatch m;
    bool found = regex_search(data, m, regex("<(.*)>(.*)</(\\1)>"));
    print_match_details(found, m);

    // We used regex_search, so we can match anywhere in the data.
    // We could use regex_match, which would only match the entire string,
    // so suffix and prefix would be empty.

    cout << "\nusing_the_match_results() ends\n";
}

void finding_repeated_matches()
{
    cout << "\nfinding_repeated_matches() starts:\n";

    // n.b. The fact that the data is on multiple lines means we don't match the
    // "person" tag. If it was on one line, change the middle of the regex to
    // "[^>]*" which means "anything but >".
    string data = "<person>\n"
        " <first>Nico</first>\n"
        " <last>Josuttis</last>\n"
        "</person>\n";

    regex reg("<(.*)>(.*)</(\\1)>");

    // Iterate over all matches by increasing the start position.
    auto pos = data.cbegin();
    auto end = data.cend();
    smatch m;
    for ( ; regex_search(pos, end, m, reg); pos = m.suffix().first)
    {
        cout << "match: " << m.str() << endl;
        cout << " tag: " << m.str(1) << endl;
        cout << " value: " << m.str(2) << endl;
    }

    // We can also iterate over matches...by using iterators.
    cout << "\nAnd now using iterators:\n";
    for (sregex_iterator it(data.begin(), data.end(), reg), end_it; it != end_it; ++it)
    {
        cout << it->str() << "\n";
    }

    cout << "\nfinding_repeated_matches() ends\n";
}

void tokenizing1()
{
    // After Josuttis, p728.
    cout << "\ntokenizing1() starts:\n";

    string data = "<person>\n"
        " <first>Nico</first>\n"
        " <last>Josuttis</last>\n"
        "</person>\n";
    regex reg("<(.*)>(.*)</(\\1)>");

    // Iterate over all matches.
    sregex_token_iterator pos(data.cbegin(), data.cend(),
                              reg,      // token separator
                              {0, 2});  // 0: full match, 2: second substring
    sregex_token_iterator end;
    for ( ; pos != end ; ++pos)
    {
        cout << "match: " << pos->str() << endl;
    }

    cout << "\ntokenizing1() ends\n";
}

void tokenizing2()
{
    // After Josuttis, p727.
    // How to process the contents between matched expressions.
    cout << "\ntokenizing2() starts:\n";

    string names = "nico, jim, helmut, paul, tim, john paul, rita";
    regex sep("[ \t\n]*[,;.][ \t\n]*"); // separated by , ; or . and spaces

    sregex_token_iterator p(names.cbegin(), names.cend(),
                            sep, // separator
                            -1); // -1: values between separators
    sregex_token_iterator e;
    for ( ; p != e; ++p)
    {
        cout << "name: " << *p << endl;
    }

    cout << "\ntokenizing2() ends\n";
}

void replacing()
{
    cout << "\nreplacing() starts:\n";

    regex r("what");
    string input = "I'll tell you what";
    string result = regex_replace(input, r, "something");
    cout << "\nresult = " << result << "\n";

    // See http://www.cplusplus.com/reference/regex/regex_replace/
    // Groups are numbered 1..n.  $` is the prefix, $' is the suffix.`
    regex r2("(te)(ll)");
    result = regex_replace(input, r2, "$2$1");  // swap "te" and "ll".
    cout << "\nresult = " << result << "\n";

    result = regex_replace(input, r2, "$2$1", format_no_copy);
    cout << "\nresult = " << result << "\n";

    result = regex_replace(input, r2, "$'$2$1$`", format_no_copy);
    cout << "\nresult = " << result << "\n";

    cout << "\nreplacing() ends\n";
}

int main(int argc, char *argv[])
{
    // The default regex grammar, enhanced ECMAScript, is by far the most
    // powerful one - see Josuttis p.

    searching();
    checking_for_a_match();
    using_the_match_results();
    finding_repeated_matches();
    tokenizing1();
    tokenizing2();
    replacing();
    return 0;
}
