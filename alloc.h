#pragma once
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static inline void *checked_malloc(size_t size) {
    void *p = malloc(size);
    if (!p) {
        fputs("[!] critical error: out of memory\n", stderr);
        exit(EXIT_FAILURE);
    }
    return p;
}
#define malloc checked_malloc

void *grow_array_using_realloc(void *arr, uint32_t *size, size_t target_size) {
    if (target_size > UINT32_MAX) abort();

    uint32_t n = *size;
    while (target_size > n) {
        uint32_t m = (n + 1) * 3 / 2;

        if (m < n)
            n = 0xffffffff;
        else 
            n = m;
    
        if (n < 16) n = 16;
    }
    char *next_array = realloc(arr, n);
    memset(next_array + *size, 0, n - *size);
    *size = n;
    return next_array;
}

static inline void *grow_array(void *a, uint32_t *size, size_t target_size) {
    uint32_t n = *size;
    if (target_size <= n)
        return a;
    return grow_array_using_realloc(a, size, target_size);
}
