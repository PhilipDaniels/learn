#pragma once

#include <cctype>
#include <cstddef>
#include <iostream>
#include <string>


// Define a special case-insensitive string type, Josuttis 690.

// Override some of the members of char_traits so that strings
// behave in a case-insensitive way.
struct case_insensitive_traits : public std::char_traits<char>
{
    static bool eq(const char& c1, const char& c2)
    {
        return std::toupper(c1) == std::toupper(c2);
    }

    static bool lt(const char& c1, const char& c2)
    {
        return std::toupper(c1) < std::toupper(c2);
    }

    static int compare(const char* s1, const char* s2, std::size_t n)
    {
        for (std::size_t i = 0; i < n; ++i)
        {
            if (!eq(s1[i], s2[i]))
                return lt(s1[i], s2[i]) ? -1 : 1;
        }

        return 0;
    }

    static const char* find(const char* s, std::size_t n, const char& c)
    {
        for (std::size_t i = 0; i < n; ++i)
        {
            if (eq(s[i], c))
                return &(s[i]);
        }

        return 0;
    }
};


typedef std::basic_string<char, case_insensitive_traits> cistring;

inline
std::ostream& operator<<(std::ostream& os, const cistring& s)
{
    // Simply convert to a normal string and then output it.
    os << std::string(s.data(), s.length());
    return os;
}
