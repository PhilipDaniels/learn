#include <iostream>

using std::cout;
using std::endl;

int main(int argc, char *argv[])
{
    // Identifiers are case-sensitive.
    int name, Name, NAME;

    // Can contain, letters, numbers and underscore.
    int there_is_no_limit_to_length_22222;

    // Illegal identifiers:
    // __foo - beginning with OR CONTAINING 2 successive underscores
    // foo__bar
    // _Foo - an underscore followed by a capital letter

    cout << "Guidance:\n"
	    "  * Never start identifiers with underscores.\n"
	    "  * Never use two successive underscores in an identifier.\n"
    	    "  * Never use a '_t' suffix on a type name.\n\n"
	    "  * DO name variables in_lower_case\n"
	    "  * DO names classes LikeThisInCamelCase\n\n"
            "  * See also http://stackoverflow.com/questions/228783/what-are-the-rules-about-using-an-underscore-in-a-c-identifier\n";
}

// Also illegal - identifiers outside functions cannot begin with an underscore.
//int _foo;

