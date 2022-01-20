#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int readlines(char ***lineptr, char *input) {
  FILE *fp = fopen(input, "r");
  if (fp == NULL) {
    perror(input);
    return 0;
  }

  int maxsize = 1;
  char **lines = malloc(maxsize * sizeof(char *));
  if (lines == NULL) {
    perror("failed to reallocate memory");
    return 0;
  }

  int n = 0;
  char *line = NULL;
  size_t linecap = 0;
  while (getline(&line, &linecap, fp) > 0) {
    // Remove the newline from the end of the string.
    line[strcspn(line, "\n")] = '\0';
    // Make sure there's enough space to copy the string.
    lines[n] = malloc(linecap);
    // We cannot directly assign `line` here as with every iteration, the `line`
    // variable is updated by `getline`.
    strcpy(lines[n++], line);
    if (n >= maxsize) {
      // Reallocate to 2 times the current size.
      maxsize *= 2;
      lines = realloc(lines, maxsize * sizeof(char *));
      if (lines == NULL) {
        perror("failed to reallocate memory");
        return 0;
      }
    }
  }

  *lineptr = lines;
  return n;
}
