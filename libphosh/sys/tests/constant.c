// Generated by gir (https://github.com/gtk-rs/gir @ 5223ce91b97a)
// from ../.. (@ eb5ea3018a51+)
// from ../../gir-files (@ 6cd7b656acd6)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) PHOSH_LOCKSCREEN_PAGE_EXTRA);
    PRINT_CONSTANT((gint) PHOSH_LOCKSCREEN_PAGE_INFO);
    PRINT_CONSTANT((gint) PHOSH_LOCKSCREEN_PAGE_UNLOCK);
    return 0;
}
