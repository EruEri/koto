

#include "date.h"
#include <stdlib.h>


Date date_now(){
    time_t t = time(0);
    struct tm* tmd = localtime(&t);
    Date d;
    d.day = tmd->tm_mday ;
    d.month = tmd->tm_mon +1;
    d.year = tmd->tm_year + 1900;
    free(tmd);
    return d;
}