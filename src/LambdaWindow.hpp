#pragma once

#include <X11/Xlib.h>

class LambdaWindow {
 private:
    Window window;
    int workspace;

 public:
     explicit LambdaWindow(Window window, int workspace = 1);
     ~LambdaWindow();
};

