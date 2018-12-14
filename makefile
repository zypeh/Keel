SOURCES=$(wildcard *.c)
HEADERS=$(wildcard *.h)
CFLAGS=-Wall -Wextra -Werror

keelc: $(SRC) $(HEADERS)
	clang $(CFLAGS) -o "$@" $(SOURCES)

clean: rm -f keelc
