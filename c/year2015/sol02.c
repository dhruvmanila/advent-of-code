#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "../lib/read.h"

// PRINT_BOX is a macro to print the box_t struct for debugging purposes.
#define PRINT_BOX(b)                                                           \
  fprintf(stdout, "box{length: %d width: %d height: %d}\n", b.length, b.width, \
          b.height)

#define MIN(a, b) ((a < b) ? a : b)

typedef struct {
  int length;
  int width;
  int height;
} box_t;

static int wrapping_paper_area(box_t box) {
  int lw = box.length * box.width;
  int wh = box.width * box.height;
  int hl = box.height * box.length;
  return 2 * lw + 2 * wh + 2 * hl + MIN(MIN(lw, wh), hl);
}

static int ribbon_length(box_t box) {
  int length = 0;
  int lw_min = MIN(box.length, box.width);
  if (lw_min == box.length) {
    length = lw_min * 2 + MIN(box.width, box.height) * 2;
  } else {
    length = lw_min * 2 + MIN(box.length, box.height) * 2;
  }
  return box.length * box.width * box.height + length;
}

int year2015_sol02(char *input) {
  char **lines = NULL;
  ssize_t lineslen = readlines(&lines, input);
  if (lineslen == -1) {
    perror(input);
    return EXIT_FAILURE;
  }

  int area = 0, ribbon = 0;
  for (int i = 0; i < lineslen; i++) {
    box_t box;
    sscanf(lines[i], "%dx%dx%d", &box.length, &box.width, &box.height);
    area += wrapping_paper_area(box);
    ribbon += ribbon_length(box);
  }

  printf("2.1: %d\n2.2: %d\n", area, ribbon);
  return EXIT_SUCCESS;
}
