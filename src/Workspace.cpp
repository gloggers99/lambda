#include "Workspace.hpp"

void Workspace::setName(std::string name) {
    this->name = name;
}

void Workspace::setName(char *name) {
    this->name = std::string(name);
}

void Workspace::addWindow(LambdaWindow window) {
    this->windows.push_back(window);
}

void Workspace::addWindows(std::vector<LambdaWindow> windows) {
    for (LambdaWindow window : windows) {
        this->windows.push_back(window);
    }
}
