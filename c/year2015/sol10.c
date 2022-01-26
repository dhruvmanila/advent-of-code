#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

const char *INPUT = "1113122113";

int year2015_sol10() {
  int seqlen;         // current sequence length
  int maxsize;        // maxsize of next sequence buffer
  int len;            // current length of next sequence buffer
  char *seq = NULL;   // current sequence
  char *next = NULL;  // next sequence
  char *end = NULL;   // end pointer for the sequence
  char prev;          // previous character
  char nprev;         // prev character count

  seqlen = strlen(INPUT);
  seq = calloc(seqlen + 1, sizeof(char));  // +1 for '\0'
  if (seq == NULL) {
    goto error;
  }
  strcpy(seq, INPUT);

  for (int i = 0; i < 50; i++) {
    maxsize = seqlen + 1;
    next = calloc(maxsize, sizeof(char));
    if (next == NULL) {
      goto error;
    }

    len = 0;
    prev = *seq;
    nprev = '1';
    end = seq + seqlen;

    do {
      seq++;
      if (*seq == prev) {
        nprev++;
        continue;
      }
      if (maxsize - len < 3) {
        maxsize *= 2;
        next = realloc(next, maxsize * sizeof(char));
        if (next == NULL) {
          goto error;
        }
      }
      next[len++] = nprev;
      next[len++] = prev;
      prev = *seq;
      nprev = '1';
    } while (seq < end);

    seq = next;
    seq[len + 1] = '\0';
    seqlen = len;

    if (i == 39) {
      printf("10.1: %d\n", seqlen);
    }
  }

  free(next);
  printf("10.2: %d\n", seqlen);
  return EXIT_SUCCESS;

error:
  if (seq != NULL) {
    free(seq);
  }
  if (next != NULL) {
    free(next);
  }
  perror(NULL);
  return EXIT_FAILURE;
}
