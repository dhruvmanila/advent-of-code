#include <stdio.h>
#include <stdlib.h>

#include "../lib/read.h"

static int combinations1(int *containers, int target, size_t len) {
  if (target == 0) {
    return 1;
  } else if (target < 0 || len == 0) {
    return 0;
  }
  return combinations1(containers + 1, target - *containers, len - 1) +
         combinations1(containers + 1, target, len - 1);
}

int year2015_sol17(char *input) {
  char **lines = NULL;
  ssize_t line_cnt = readlines(&lines, input);
  if (line_cnt == -1) {
    perror(input);
    return EXIT_FAILURE;
  }

  int *containers = calloc(line_cnt, sizeof(int));
  if (containers == NULL) {
    perror(NULL);
    return EXIT_FAILURE;
  }

  for (int i = 0; i < line_cnt; i++) {
    containers[i] = atoi(lines[i]);
  }

  printf("17.1: %d\n", combinations1(containers, 150, line_cnt));
  free(containers);
  return EXIT_SUCCESS;
}
