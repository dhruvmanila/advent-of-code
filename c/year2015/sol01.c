#include <stdint.h>
#include <stdio.h>
#include <string.h>

void year2015_sol01(char *input) {
  FILE *fp = fopen(input, "r");
  if (fp == NULL) {
    fprintf(stderr, "unable to open file: %s\n", input);
  }

  char *line = NULL;
  size_t linecap = 0;
  getline(&line, &linecap, fp);
  fclose(fp);

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
}
