CC := clang
CFLAGS += -O2 -Wall -Wextra -Werror -Wshadow -pedantic
BIN = aoc

SUBDIRS := $(wildcard ./*/.)

.PHONY: all
all: $(SUBDIRS) $(BIN)

.PHONY: $(SUBDIRS)
$(SUBDIRS):
	$(MAKE) -C $@

$(BIN): main.o
	$(CC) $(CFLAGS) -o $@ $(wildcard year2015/*.o) $^

%.o: %.c
	$(CC) $(CFLAGS) -c $<

.PHONY: clean
clean: ## Remove build artifacts
	rm $(BIN) *.o **/*.o
