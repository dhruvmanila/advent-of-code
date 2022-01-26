#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../lib/read.h"

const char *input = "hxbxwxba";

int year2015_sol11() {
  int len = strlen(input);
  char *pass = malloc((len + 1) * sizeof(char));
  if (pass == NULL) {
    perror(NULL);
    return EXIT_FAILURE;
  }
  strlcpy(pass, input, len + 1);

  int k = 1;
  int i;
  int pair;      // number of non-overlapping pairs
  bool triplet;  // are there any triplet?
  while (true) {
    // Update all the 'z' to 'a' from the right until there's a character other
    // than 'z'. Then, update the ith character. This is incrementing the
    // password string by one value and wrapping around at 'z'.
    for (i = len - 1; i >= 0 && pass[i] == 'z'; i--) {
      pass[i] = 'a';
    }
    pass[i]++;

    pair = 0;
    triplet = false;
    for (i = 0; i < len; i++) {
      if (strchr("iol", pass[i])) {  // password may not contain 'i', 'o' or 'l'
        break;
      }
      pair +=
          ((i > 1 && pass[i - 2] == pass[i - 1] && pass[i - 1] != pass[i]) ||
           (i == len - 1 && pass[i - 1] == pass[i]));
      triplet |= (i > 1 && pass[i - 2] + 1 == pass[i - 1] &&
                  pass[i - 1] + 1 == pass[i]);
    }

    if (i == len && pair >= 2 && triplet) {
      printf("11.%d: %s\n", k, pass);
      if (k == 2) {
        break;
      }
      k++;
    }
  }

  free(pass);
  return EXIT_SUCCESS;
}
