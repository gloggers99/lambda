#include "LambdaWindow.hpp"

LambdaWindow::LambdaWindow(Window window, int workspace) {
    this->window = window;
    this->workspace = workspace;
}

LambdaWindow::~LambdaWindow() = default;
