#include <errno.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

#include "alloc.h"
#include "parser.h"

int main(int argc, char const *argv[]) {
        if (argc != 2) {
                fputs("* keelc <filename>", stderr);
                exit(EXIT_FAILURE);
        }

        FILE *fp = fopen(argv[1], "r");

        if (ferror(fp)) {
                fputs("I/O error when reading", stderr);
                exit(EXIT_FAILURE);
        }

        char *content = 0;
        size_t len = 0;
        size_t offset = 0;
        while (true) {
                content = grow_array(content, &len, len + 256);
                size_t n = fread(content + offset, 1, len - offset, fp);
                if (n < len - offset) {
                        content[offset + n] = '\0';
                        break;
                }
                offset += n;
        }

        SrcFile src = {.filepath = (uint8_t *) argv[1], .buf = (uint8_t *) content, .len = len};

        SrcScanner sc;
        new_scanner(&sc, &src);
        while (true) {
                Token t = scan(&sc);
                if (t == EndOfFile)
                        break;
                printf("[%d]", t);
        }

        free(content);
        fclose(fp);
        return EXIT_SUCCESS;
}
