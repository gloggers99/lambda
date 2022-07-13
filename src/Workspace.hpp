#pragma once

#include <string>
#include <vector>

#include "LambdaWindow.hpp"

class Workspace {
 private:
     std::vector<LambdaWindow> windows;
     std::string name;
 public:
     void setName(std::string name);
     void setName(char *name);
     void addWindow(LambdaWindow window);
     void addWindows(std::vector<LambdaWindow> windows);

     Workspace() = default;
     ~Workspace() = default;
};
