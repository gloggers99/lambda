
#include <X11/Xlib.h>

#include <iostream>
#include <vector>
#include <string>

#include "Workspace.hpp"
#include "LambdaWindow.hpp"
#include "Config.hpp"

class WindowManager {
 private:
     static int errorHandler(Display *display, XErrorEvent *e) {
         if (e->error_code == BadAccess) {
             std::cerr << "Another WM is already running!\n";
         } else {
             std::cerr << "An X11 Error was triggered: " << e->error_code << "\n";
         }

         return 0;
     }

     Display *display;
     Window root;

     std::vector<Workspace> workspaces;
     int currentWorkspace = 0;
     int numberOfWorkspaces = 0;

 public:
    explicit WindowManager(Config config);
    void run();
    ~WindowManager();
};

