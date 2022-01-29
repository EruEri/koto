#ifndef DATE_H
#define DATE_H


#include <stdint.h>
#include <time.h>

typedef struct {
    uint8_t day;
    uint8_t month;
    int16_t year;
} Date;

Date date_now();

#endif