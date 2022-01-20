#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int year2015_sol01(char *input) {
  FILE *fp = fopen(input, "r");
  if (fp == NULL) {
    perror(input);
    return EXIT_FAILURE;
  }

  char *line = NULL;
  size_t linecap = 0;
  getline(&line, &linecap, fp);

  if (fclose(fp)) {
    perror(input);
    return EXIT_FAILURE;
  };

  int16_t floor = 0;
  uint16_t position = 0;
  for (uint16_t i = 0; i < (uint16_t)strlen(line); i++) {
    switch (line[i]) {
    case '(': floor++; break;
    case ')': floor--; break;
    }
    if (floor == -1 && position == 0) {
      position = i + 1;
    }
  }

  printf("1.1: %d\n1.2: %d\n", floor, position);
  return EXIT_SUCCESS;
}
