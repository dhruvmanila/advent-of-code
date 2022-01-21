#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../lib/read.h"

int year2015_sol01(char *input) {
  char **lines = NULL;
  ssize_t linelen = readlines(&lines, input);
  if (linelen == -1) {
    perror(input);
    return EXIT_FAILURE;
  }
  char *line = lines[0];  // one line file

  int16_t floor = 0;
  uint16_t position = 0;
  for (uint16_t i = 0; i < (uint16_t)strlen(line); i++) {
    switch (line[i]) {
      case '(':
        floor++;
        break;
      case ')':
        floor--;
        break;
    }
    if (floor == -1 && position == 0) {
      position = i + 1;
    }
  }

  printf("1.1: %d\n1.2: %d\n", floor, position);
  return EXIT_SUCCESS;
}
