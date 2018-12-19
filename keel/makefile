SOURCES=$(wildcard *.c)
HEADERS=$(wildcard *.h)
CFLAGS=-O3 -Wall -Wextra -Werror -Wshadow -pedantic

keelc: $(SRC) $(HEADERS)
	clang $(CFLAGS) -o "$@" $(SOURCES)

