#pragma once

#include <functional>

// A general purpose hashing library, based on Josuttis p364 which is in turn
// based on code from Boost. The default hash function, hash<>, is in
// <functional> and provides hashing for common types: all integral types, all
// floating-point types, pointers, strings, error_code, thread::id, bitset<> and
// vector<bool>.


// From boost (functional/hash)
// see http://www.boost.org/doc/libs/1_35_0/doc/html/hash/combine.html
template <typename T>
inline void hash_combine(std::size_t& seed, const T& val)
{
    seed ^= std::hash<T>()(val) + 0x9e3779b9 + (seed << 6) + (seed >> 2);
}


// Auxiliary generic functions to create a hash value using a seed.
template <typename T>
inline void hash_val(std::size_t& seed, const T& val)
{
    hash_combine(seed, val);
}

template <typename T, typename... Types>
inline void hash_val(std::size_t& seed, const T& val, const Types&... args)
{
    hash_combine(seed, val);
    hash_val(seed, args...);
}

// auxiliary generic function to create a hash value out of a heterogeneous list
// of arguments.
template <typename... Types>
inline std::size_t hash_val(const Types&... args)
{
    std::size_t seed = 0;
    hash_val(seed, args...);
    return seed;
}
