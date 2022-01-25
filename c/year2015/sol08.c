#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../lib/read.h"

void parsestr(const char *s, size_t *cnt1, size_t *cnt2) {
  size_t memlen = 0;              // number of characters in-memory
  size_t len = strlen(s);         // number of characters of code
  const char *end = s + len - 1;  // skip the last quote char
  s++;                            // skip the leading quote char

  // Starting and ending representating quotes: 2
  // Starting and ending escaped quotes: 4
  //      "" -> "\"\""
  size_t enclen = 6;  // number of characters in encoded string

  while (s < end) {
    if (*s != '\\') {
      s++;
      memlen++;
      enclen++;
      continue;
    }
    s++;          // skip the escape character
    enclen += 2;  // escape character: '\\'
    switch (*s++) {
      case '\\':
      case '\"':
        memlen++;
        enclen += 2;
        break;
      case 'x':
        s += 2;
        memlen++;
        enclen += 3;  // escape char 'x' + two hex digits
    }
  }

  *cnt1 += len - memlen;
  *cnt2 += enclen - len;
}

int year2015_sol08(char *input) {
  char **lines = NULL;
  ssize_t line_cnt = readlines(&lines, input);
  if (line_cnt == -1) {
    perror(input);
    return EXIT_FAILURE;
  }

  size_t cnt1 = 0;
  size_t cnt2 = 0;
  for (int i = 0; i < line_cnt; i++) {
    parsestr(lines[i], &cnt1, &cnt2);
  }

  printf("8.1: %zu\n8.2: %zu\n", cnt1, cnt2);
  return EXIT_SUCCESS;
}
