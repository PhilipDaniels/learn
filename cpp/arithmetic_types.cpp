#include <cstdlib>
#include <iostream>
#include <string>
#include <iomanip>

using std::cout;
using std::endl;
using std::string;
using std::setw;
using std::left;

void out(const string &desc, const string &meaning, int size, const string& min_size)
{
    cout << left << setw(13) << desc
	 << setw(35) << meaning
	 << setw(5) << size
	 << setw(10) << min_size
	 << endl;
}

int main(int argc, char *argv[])
{
    cout << "char16_t and char32_t support is limited - for example, there are\n"
	 << "corresponding string types (string, wstring, u16string, u32string)\n"
	 << "but IO support and conversions are not in the standard library.\n"
	 << "There are also fixed-width integer types such as int16_t in <cstdint>\n\n"
	 << "Guidance: (Based on Lippman and the Google Style Guide)\n"
	 << "  * Use char or wchar_t and string or wstring.\n"
	 << "  * Use double because float is not precise enough and long double is slow.\n\n"
	 << "  * ONLY use int - you can assume it will be 32 bits.\n"
	 << "  * UNLESS your ints are very large - then use int64_t.\n\n"
	 << "  * Avoid unsigned int because of overflow errors.\n"
	 << "  * Avoid char, bool or other int types in arithmetic expressions\n"
	 << "    because of type promotion.\n\n"
	 << "See also: https://en.wikipedia.org/wiki/C_data_types\n"
	 << "and https://google.github.io/styleguide/cppguide.html#Integer_Types\n\n"
         << "Size is the size on this machine in bytes.\n"
	 << "MinSize is the minimum size guaranteed by the standard.\n"
	 << endl;
    
    cout << "Type         Meaning                            Size MinSize" << endl;
    cout << "====         =======                            ==== =======" << endl;
    
    out("bool", "boolean", sizeof(bool), "na");
    out("char", "8 bit chars", sizeof(char), "8 bits");
    out("wchar_t", "Wide char", sizeof(wchar_t), "16 bits");
    out("char16_t", "UCS-2 Unicode", sizeof(char16_t), "16 bits");
    out("char32_t", "UCS-4/UTF-32 Unicode", sizeof(char32_t), "32 bits");
    cout << endl;
    out("short", "short integer", sizeof(short), "16 bits");
    out("int", "integer", sizeof(int), "16 bits");
    out("long", "long integer", sizeof(long), "32 bits");
    out("long long", "long integer", sizeof(long long), "64 bits");
    out("float", "single-precision floating point", sizeof(float), "6 significant digits");
    out("double", "double-precision floating point", sizeof(double), "10 significant digits");
    out("long double", "extended-precision floating point", sizeof(long double), "10 significant digits");
}
