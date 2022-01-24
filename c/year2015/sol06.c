#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../lib/read.h"

#define MAX(a, b) ((a) > (b) ? (a) : (b))

typedef enum {
  ON,
  OFF,
  TOGGLE,
} op_t;

int year2015_sol06(char *input) {
  char **lines = NULL;
  ssize_t line_cnt = readlines(&lines, input);
  if (line_cnt == -1) {
    perror(input);
    return EXIT_FAILURE;
  }

  // grid could be made static to intialize it with zeroes.
  // https://stackoverflow.com/q/2589749

  // lights all start at zero
  uint8_t grid1[1000][1000] = {0};  // part one grid
  uint8_t grid2[1000][1000] = {0};  // part two grid

  for (char *line = *lines; line; line = *++lines) {
    op_t op;
    int x0, y0;  // start range
    int x1, y1;  // end range

    char *instruction = strsep(&line, " ");
    if (!strcmp(instruction, "toggle")) {  // strcmp returns 0 if equal
      op = TOGGLE;
    } else {
      char *turn = strsep(&line, " ");
      op = !strcmp(turn, "on") ? ON : OFF;  // strcmp returns 0 if equal
    }
    sscanf(line, "%d,%d through %d,%d", &x0, &y0, &x1, &y1);

    for (int x = x0; x <= x1; x++) {
      for (int y = y0; y <= y1; y++) {
        switch (op) {
          case ON:
            grid1[y][x] = 1;
            grid2[y][x]++;
            break;
          case OFF:
            grid1[y][x] = 0;
            grid2[y][x] = MAX(0, grid2[y][x] - 1);
            break;
          case TOGGLE:
            grid1[y][x] = !grid1[y][x];
            grid2[y][x] += 2;
            break;
        }
      }
    }
  }

  int on_cnt1 = 0;
  int on_cnt2 = 0;
  for (int x = 0; x < 1000; x++) {
    for (int y = 0; y < 1000; y++) {
      on_cnt1 += grid1[y][x];
      on_cnt2 += grid2[y][x];
    }
  }

  printf("6.1: %d\n6.2: %d\n", on_cnt1, on_cnt2);
  return EXIT_SUCCESS;
}
