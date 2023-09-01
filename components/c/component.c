#include "greeter_adapter.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void exports_wasmcon2023_greet_interface_greet(greeter_adapter_string_t *ret) {
    greeter_adapter_string_t greeting;
    wasmcon2023_greet_interface_greet(&greeting);
    greeter_adapter_string_t a;
    greeter_adapter_string_set(&a, " and C!");

    // TODO: try without extra intermediate string
    // char* suffix = " and C!";
    // size_t suffix_len = strlen(suffix);

    ret->len = greeting.len + a.len;
    ret->ptr = malloc(ret->len * 2); // TODO: test with +1 instead of *2
    memset(ret->ptr, 0, ret->len * 2);

    memcpy(ret->ptr, greeting.ptr, greeting.len);
    memcpy(ret->ptr + greeting.len, a.ptr, a.len);

    greeter_adapter_string_free(&a);
    greeter_adapter_string_free(&greeting);
}