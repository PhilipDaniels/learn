#pragma once

#include <functional>
#include <string>
#include <vector>

// A type for a callable that compares two characters.
typedef std::function<bool(char x, char y)> CharComparison;

// A pair of default CharComparison functions.
bool charcmp(char x, char y)
{
    return x == y;
}

bool icharcmp(char x, char y)
{
    return tolower(x) == tolower(y);
}

const std::string whitespace(" \n\r\t");

bool starts_with(const std::string& haystack,
                 const std::string& needle,
                 CharComparison cmp = charcmp);

bool ends_with(const std::string& haystack,
               const std::string& needle,
               CharComparison cmp = charcmp);

std::string::const_iterator find(const std::string& haystack,
                                 const std::string& needle,
                                 CharComparison cmp = charcmp);

std::string& ltrim(std::string& value, const std::string& trim_chars = whitespace)
{
    value.erase(0, value.find_first_not_of(trim_chars));
    return value;
}

std::string& rtrim(std::string& value, const std::string& trim_chars = whitespace)
{
    value.erase(value.find_last_not_of(trim_chars) + 1);
    return value;
}

std::string& trim(std::string& value, const std::string& trim_chars = whitespace)
{
    rtrim(value, trim_chars);
    ltrim(value, trim_chars);
    return value;
}

// TODO: Write these.
//std::string& replace(std::string& value, const std::string& old, const std::string& new);
//std::string& replace_all(std::string& value, const std::string& old, const std::string& new);


template <class ContainerT>
void split(const std::string& str,               // Split this string
           ContainerT& tokens,                   // Placing results in this container
           const std::string& delimiters = " ",  // Splitting on these delimiters
           bool trimEmpty = false)               // Whether to skip empty splits.
{
    std::string::size_type pos, lastPos = 0;

    using value_type = typename ContainerT::value_type;
    using size_type  = typename ContainerT::size_type;

    while (true)
    {
        pos = str.find_first_of(delimiters, lastPos);
        if (pos == std::string::npos)
        {
            pos = str.length();
            if (pos != lastPos || !trimEmpty)
                tokens.push_back(value_type(str.data() + lastPos, (size_type)pos - lastPos));
            break;
        }
        else
        {
            if (pos != lastPos || !trimEmpty)
                tokens.push_back(value_type(str.data() + lastPos, (size_type)pos - lastPos));
        }

        lastPos = pos + 1;
    }
}


