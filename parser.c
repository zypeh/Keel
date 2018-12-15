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
    uint8_t c = src[scanner->pos];

    /* End of file */
    if (scanner->pos >= scanner->src->len || c == '\0') {
        return EndOfFile;
    }

    switch (c) {
        case LINEFEED:
            scanner->line++;
            scanner->col = 0;
            next(scanner, 2);
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
            while (scanner->pos <= scanner->src->len || src[scanner->pos] != '\0') {
                if (src[scanner->pos] == SPACE ||
                    src[scanner->pos] == TAB ||
                    src[scanner->pos] == FORMFEED ||
                    src[scanner->pos] == VERTICALTAB ||
                    (src[scanner->pos] == CARRIAGERETURN &&
                     src[scanner->pos + 1] != LINEFEED)) {
                    next(scanner, 1);
                }
                break;
            }
            return Whitespace;

        case SLASH:
            if (src[scanner->pos + 1] == SLASH) {
                next(scanner, 2);
                while (scanner->pos <= scanner->src->len || src[scanner->pos] != '\0') {
                    if (src[scanner->pos] == Newline) {
                        // TODO: saves comment content
                        break;
                    }
                    next(scanner, 1);
                }
            }
            return Comment;

        default:
            putchar(c);
            next(scanner, 1);
            return Unknown;
    }
}
