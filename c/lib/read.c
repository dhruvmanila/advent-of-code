#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// readlines reads all the lines from input into *lineptr. The caller should
// not provide a malloced buffer for *lineptr as this function will override
// whatever is in *lineptr with the lines from the input.
//
// The function returns the number of lines read or -1 if an error occurs.
// This function may fail due to any errors specified for fopen(), malloc(),
// realloc() and getline().
ssize_t readlines(char ***lineptr, char *input) {
  FILE *fp = fopen(input, "r");
  if (fp == NULL) {
    return -1;
  }

  ssize_t result = -1; // return value
  ssize_t maxsize = 1; // current maximum size of lines pointer
  ssize_t cur_len = 0; // current length of lines pointer

  char **lines = malloc(maxsize * sizeof(char *));
  if (lines == NULL) {
    goto fclose_return;
  }

  char *line = NULL;
  size_t linecap = 0;
  ssize_t linelen;
  while ((linelen = getline(&line, &linecap, fp)) > 0) {
    // Remove the newline from the end of the string.
    line[strcspn(line, "\n")] = '\0';
    // Make sure there's enough space to copy the string.
    lines[cur_len] = malloc(linelen);
    if (lines[cur_len] == NULL) {
      goto fclose_return;
    }
    // We cannot directly assign `line` here as with every iteration, the `line`
    // variable is updated by getline().
    strcpy(lines[cur_len++], line);
    if (cur_len >= maxsize) {
      // Reallocate to 2 times the current size.
      maxsize *= 2;
      lines = realloc(lines, maxsize * sizeof(char *));
      if (lines == NULL) {
        goto fclose_return;
      }
    }
  }

  // Override the *lineptr with the lines read.
  *lineptr = lines;
  result = cur_len;

fclose_return:
  fclose(fp);
  return result;
}
