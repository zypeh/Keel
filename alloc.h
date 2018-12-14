#pragma once
#include <stdio.h>
#include <stdlib.h>

static inline void *checked_malloc(size_t size) {
    void *p = malloc(size);
    if (!p) {
        fputs("[!] critical error: out of memory\n", stderr);
        exit(EXIT_FAILURE);
    }
    return p;
}

#define malloc checked_malloc

void *grow_array_using_realloc(void *arr, uint32_t *size, size_t target_size);
static inline void *grow_array(void *a, uint32_t *size, size_t target_size) {
    uint32_t n = *size;
    if (target_size <= n)
        return a;
    return grow_array_using_realloc(a, size, target_size);
}
