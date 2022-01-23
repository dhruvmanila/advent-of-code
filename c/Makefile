CC := clang
CFLAGS += -Wall -Wextra
LDFLAGS += -lssl -lcrypto
BIN = aoc

# No optimizations when debugging
ifeq ($(strip $(DEBUG)), 1)
	CFLAGS += -g
else
	CFLAGS += -O3
endif

SRC     = $(shell find . -type f -name "*.c")
OBJECTS = $(SRC:.c=.o)
HEADERS = $(shell find . -type f -name "*.h")

$(BIN): $(OBJECTS)
	$(CC) $(LDFLAGS) -o $@ $^

$(OBJECTS): %.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@
$(OBJECTS): .EXTRA_PREREQS = $(HEADERS)

.PHONY: clean
clean: ## Remove build artifacts
	-rm $(BIN) *.o **/*.o
