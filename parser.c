#include <stdint.h>
#include <stdio.h>

#include "parser.h"

void new_scanner(srcScanner* scanner, srcFile* source) {
    scanner->src = source;
    scanner->pos = 0;
    scanner->line = 0;
    scanner->col = 0;
    // fwrite(scanner->src->buf, scanner->src->len, 1, stdout);
}

void next(srcScanner* scanner, int step) {
    scanner->pos += step;
    scanner->col += step;
}

Token scan(srcScanner* scanner) {
    /* End of file */
    if (scanner->pos >= scanner->src->len || scanner->src->buf[scanner->pos] == 0) {
        return EndOfFile;
    }

    uint8_t* src = scanner->src->buf;
    uint8_t c = src[scanner->pos];

    switch (c) {
        case HASH:
            printf("<%d, %c>\n", HASH, src[scanner->pos]);
            scanner->pos++; /* TODO: make it consume and peek next n char */
            return Hash;
        case LINEFEED:
            printf("!!!\n");
            scanner->pos++; /* TODO: make it consume and peek next n char */
            return Newline;
        default:
            printf("<%d, %c>\n", Unknown, src[scanner->pos]); /* TODO: cannot print unicode characters */
            scanner->pos++; /* TODO: make it consume and peek next n char */
            return Unknown;
    }
}
