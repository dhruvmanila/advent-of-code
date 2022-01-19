CC := clang
CFLAGS += -O2 -Wall -Wextra -Werror -Wshadow -pedantic

SRC = $(wildcard *.c)
OBJ = $(SRC:.c=.o)

.PHONY: all
all: $(OBJ)

$(OBJ): $(SRC)
	$(CC) $(CFLAGS) -c $^
$(OBJ): .EXTRA_PREREQS = solutions.h
