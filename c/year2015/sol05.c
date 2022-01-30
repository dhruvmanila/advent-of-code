#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../lib/read.h"

static int is_nice_string_v1(const char *str) {
  int vowel_cnt = 0;
  int twice_cnt = 0;
  int invalid_cnt = 0;

  char prev = '\0';
  for (const char *ch = str; *ch != '\0'; ch++) {
    switch (*ch) {
      case 'a':  // fallthrough
      case 'e':  // fallthrough
      case 'i':  // fallthrough
      case 'o':  // fallthrough
      case 'u':
        vowel_cnt++;
        break;
      case 'b':  // fallthrough
      case 'd':  // fallthrough
      case 'q':  // fallthrough
      case 'y':
        if (*ch - 1 == prev) {
          invalid_cnt++;
        }
    }
    if (*ch == prev) {
      twice_cnt++;
    }
    prev = *ch;
  }

  return vowel_cnt >= 3 && twice_cnt >= 1 && invalid_cnt == 0;
}

static int is_nice_string_v2(const char *str) {
  int pair = 0;    // any non-overlapping pair
  int repeat = 0;  // any letter repeated with one letter between them
  int len = strlen(str);
  for (int i = 0; i < len; i++) {
    pair = pair || (i > 0 && i < len - 2 &&
                    !!memmem(&str[i + 1], len - i - 1, &str[i - 1], 2));
    repeat = repeat || (i > 0 && i < len - 1 && str[i - 1] == str[i + 1]);
  }
  return pair && repeat;
}

int year2015_sol05(char *input) {
  char **lines = NULL;
  ssize_t line_cnt = readlines(&lines, input);
  if (line_cnt == -1) {
    perror(input);
    return EXIT_FAILURE;
  }

  int count1 = 0, count2 = 0;
  for (int i = 0; i < line_cnt; i++) {
    if (is_nice_string_v1(lines[i])) {
      count1++;
    }
    if (is_nice_string_v2(lines[i])) {
      count2++;
    }
  }

  printf("5.1: %d\n5.2: %d\n", count1, count2);
  return EXIT_SUCCESS;
}
