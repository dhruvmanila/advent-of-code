CC := clang
CFLAGS += -O2 -Wall -Wextra -Werror -Wshadow -pedantic
BIN = aoc

SRC     = $(shell find . -type f -name "*.c")
OBJECTS = $(SRC:.c=.o)
HEADERS = $(shell find . -type f -name "*.h")

$(BIN): $(OBJECTS)
	$(CC) -o $@ $^

$(OBJECTS): %.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@
$(OBJECTS): .EXTRA_PREREQS = $(HEADERS)

.PHONY: clean
clean: ## Remove build artifacts
	-rm $(BIN) *.o **/*.o
