#include "greeter_adapter.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void exports_wasmcon2023_greet_interface_greet(greeter_adapter_string_t *ret) {
    greeter_adapter_string_t greeting;
    wasmcon2023_greet_interface_greet(&greeting);
    greeter_adapter_string_t a;
    greeter_adapter_string_set(&a, " and C!");

    ret->len = greeting.len + a.len;
    ret->ptr = malloc(ret->len + 1);

    strcpy(ret->ptr, greeting.ptr);
    strcat(ret->ptr, a.ptr);
    greeter_adapter_string_free(&a);
    greeter_adapter_string_free(&greeting);
}