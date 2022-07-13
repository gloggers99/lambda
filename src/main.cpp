#include <X11/Xlib.h>

#include <string>

#include "WindowManager.hpp"
#include "Config.hpp"
#include "KeyPair.hpp"

int main(int argc, char *argv[]) {
    WindowManager wm(Config {
                std::vector<std::string> { "www", "dev", "irc", "game" },
                std::vector<KeyPair> {
                    KeyPair { Mod1Mask | ShiftMask, "q", "quit" }
                },

                3,        // border_width
                0xff0000, // border_color

                5,        // inner_gaps
                10        // outer_gaps
            });

    wm.run();

    return 0;
}
