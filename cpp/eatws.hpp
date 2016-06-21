#pragma once

#include <iosfwd>
#include <iostream>

// This shows how to implement a custom manipulator.
// See Josuttis, p777.
template <typename charT, typename traits>
inline std::basic_istream<charT, traits>&
eatws(std::basic_istream<charT, traits>& strm)
{
    while (true)
    {
        auto c = strm.peek();
        if (c == ' ' || c == '\t' || c == '\n' || c == '\r')
        {
            std::cout << "Ignoring...\n";
            strm.ignore();
            //strm.seekg(1, std::ios::cur);
        }
        else
            break;
    }

    return strm;
}
