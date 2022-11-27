#include <stdio.h>
#include <stdint.h>

int add(int i_one, int i_two) {
    printf("[C] i_one input is: %i \n", i_one);
    printf("[C] i_two input is: %i \n", i_two);

    return i_one + i_two;
}