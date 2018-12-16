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
    uint8_t* src = scanner->src->buf;
    uint32_t len = scanner->src->len;
    uint8_t c = src[scanner->pos];

    /* End of file */
    if (scanner->pos >= len || c == '\0') {
        return EndOfFile;
    }

    switch (c) {
        case LINEFEED:
            scanner->line++;
            scanner->col = 0;
            next(scanner, 1);
            return Newline;

        case CARRIAGERETURN: /* check \r\n, if not, treat it as whitespace */
            if (src[scanner->pos + 1] == LINEFEED) {
                scanner->line++;
                scanner->col = 0;
                next(scanner, 2);
                return Newline;
            }
        case SPACE:
        case TAB:
        case FORMFEED:
        case VERTICALTAB:
            /* Trim whitespaces */
            next(scanner, 1);
            while (scanner->pos <= len || src[scanner->pos] != '\0') {
                if (src[scanner->pos] == SPACE ||
                    src[scanner->pos] == TAB ||
                    src[scanner->pos] == FORMFEED ||
                    src[scanner->pos] == VERTICALTAB ||
                    (src[scanner->pos] == CARRIAGERETURN &&
                     src[scanner->pos + 1] != LINEFEED)) {
                    next(scanner, 1);
                    continue;
                }
                break;
            }
            return Whitespace;

        case MINUS:
            if (src[scanner->pos + 1] == MINUS) {
                next(scanner, 1);
                while (scanner->pos <= len && src[scanner->pos] != '\0') {
                    if (scanner->pos + 1 <= len ||
                        src[scanner->pos + 1] != Newline ||
                        src[scanner->pos + 1] != '\0') {
                        next(scanner, 1);
                        // TODO: saves comment content
                        // putchar(src[scanner->pos]);
                        break;
                    }
                    next(scanner, 1);
                }
                return Comment;
            }
        default:
            // putchar(src[scanner->pos]);
            next(scanner, 1);
            return Unknown;
    }
}
