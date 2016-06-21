#pragma once

#include <iostream>
#include <string>

template <typename T>
void print_elements(const T& col, const std::string& optstr = "")
{
    std::cout << optstr;
    for (const auto& elem : col)
    {
        std::cout << elem << ' ';
    }
    std::cout << std::endl;
}

template <typename ForwardIt>
void print_elements(ForwardIt begin, ForwardIt end, const std::string& optstr = "")
{
    std::cout << optstr;
    while (begin != end)
    {
        const auto& elem = *begin;
        std::cout << elem << ' ';
        ++begin;
    }

    std::cout << std::endl;
}
