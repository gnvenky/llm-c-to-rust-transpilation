CC=gcc
CFLAGS=-Wall -Wextra -Iinclude

all: parser_test

parser_test: src/parser.c tests/parser_test.c
	$(CC) $(CFLAGS) -o parser_test src/parser.c tests/parser_test.c

clean:
	rm -f parser_test
# makefile
