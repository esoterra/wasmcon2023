#include "proxy_greeter.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void exports_wasmcon2023_greet_interface_greet(proxy_greeter_string_t *ret) {
    proxy_greeter_string_t greeting;
    wasmcon2023_greet_interface_greet(&greeting);
    char* suffix = " and C!";
    size_t suffix_len = strlen(suffix);

    ret->len = greeting.len + suffix_len;
    ret->ptr = malloc(ret->len);

    memcpy(ret->ptr, greeting.ptr, greeting.len);
    memcpy(ret->ptr + greeting.len, suffix, suffix_len);

    proxy_greeter_string_free(&greeting);
}