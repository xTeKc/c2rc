#include <stdio.h>
#include <stdint.h>

int add(int c_input_one, int c_input_two) {
    printf("[C] input one is: %i \n", c_input_one);
    printf("[C] input two is: %i \n", c_input_two);

    return c_input_one + c_input_two;
}

int sub(int c_input_one, int c_input_two) {
    printf("[C] input one is: %i \n", c_input_one);
    printf("[C] input two is: %i \n", c_input_two);

    return c_input_one - c_input_two;
}

int mul(int c_input_one, int c_input_two) {
    printf("[C] input one is: %i \n", c_input_one);
    printf("[C] input two is: %i \n", c_input_two);

    return c_input_one * c_input_two;
}

int div(int c_input_one, int c_input_two) {
    printf("[C] input one is: %i \n", c_input_one);
    printf("[C] input two is: %i \n", c_input_two);

    return c_input_one / c_input_two;
}