#pragma once

#include <string>
#include <vector>

#include "KeyPair.hpp"

struct Config {
    std::vector<std::string> workspaces; // { "www", "dev", "irc" }
    std::vector<KeyPair> keys;

    int border_width;
    int border_color;

    int inner_gaps;
    int outer_gaps;
};

