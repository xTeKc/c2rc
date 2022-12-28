#include <stdio.h>
#include <stdint.h>

uint32_t add(uint32_t c_input_one, uint32_t c_input_two) {
    printf("[C] input one is: %i \n", c_input_one);
    printf("[C] input two is: %i \n", c_input_two);

    return c_input_one + c_input_two;
}

uint32_t sub(uint32_t c_input_one, uint32_t c_input_two) {
    printf("[C] input one is: %i \n", c_input_one);
    printf("[C] input two is: %i \n", c_input_two);

    return c_input_one - c_input_two;
}

uint32_t mul(uint32_t c_input_one, uint32_t c_input_two) {
    printf("[C] input one is: %i \n", c_input_one);
    printf("[C] input two is: %i \n", c_input_two);

    return c_input_one * c_input_two;
}

uint32_t div(uint32_t c_input_one, uint32_t c_input_two) {
    printf("[C] input one is: %i \n", c_input_one);
    printf("[C] input two is: %i \n", c_input_two);

    return c_input_one / c_input_two;
}