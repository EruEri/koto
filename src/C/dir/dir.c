#include "dir.h"
#include <pwd.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>

char* get_home_dir(){
    struct passwd *pw = getpwuid(getuid());
    return pw->pw_dir;
}