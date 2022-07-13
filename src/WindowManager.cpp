#include "WindowManager.hpp"

WindowManager::WindowManager(Config config) {
    XSetErrorHandler(errorHandler);

    this->display = XOpenDisplay(NULL);
    this->root = XDefaultRootWindow(this->display);

    // request event access from X11
    XSelectInput(this->display,
                 this->root,
                 SubstructureNotifyMask | SubstructureNotifyMask);

    // calculate workspace information
    for (std::string str : config.workspaces) {
        // make new workspace for each one in the config
        Workspace ws;
        ws.setName(str);
        this->workspaces.push_back(ws);
    }
    this->numberOfWorkspaces = this->workspaces.size();

    std::cout << "initialized " << this->numberOfWorkspaces << " workspaces.\n";
}

void WindowManager::run() {
    XSync(this->display, false);

    XEvent event;

    for (;;) {
        XNextEvent(this->display, &event);

        switch (event.type) {
            case CreateNotify:
                break;
            case DestroyNotify:
                break;
            case KeyPress:
                break;
            case ButtonPress:
                break;
            case MapRequest:
                break;
        }
    }
}

WindowManager::~WindowManager() {
    XCloseDisplay(this->display);
}
