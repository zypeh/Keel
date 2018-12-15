SOURCES=$(wildcard *.c)
HEADERS=$(wildcard *.h)
CFLAGS=-Wall -Wextra -Werror -Wshadow

keelc: $(SRC) $(HEADERS)
	clang $(CFLAGS) -o "$@" $(SOURCES)

