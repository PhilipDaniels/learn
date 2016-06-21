#pragma once

#include <iosfwd>

// Saves the flags state of an iostream and automatically restores the flags
// upon destruction. This is handy when you are using manipulators inside
// functions, such as changing to hex mode, for example.
class IosFlagSaver
{
public:
    explicit IosFlagSaver(std::ios_base& ios)
        : ios_(ios), flags_(ios.flags())
    {
    }

    ~IosFlagSaver()
    {
        ios_.flags(flags_);
    }

    IosFlagSaver(const IosFlagSaver& rhs) = delete;
    IosFlagSaver& operator=(const IosFlagSaver& rhs) = delete;

    std::ios::fmtflags flags() const { return flags_; }

private:
    std::ios_base& ios_;
    std::ios::fmtflags flags_;
};
