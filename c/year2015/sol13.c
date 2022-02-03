#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../lib/read.h"

// This solution is similar to day 9.

// Size of the attendees buffer such that it is equal to or greater than all the
// possible unique person names for both the test and actual puzzle input.
#define BUFFER_SIZE 16  // size should be in power of 2

typedef struct {
  char name[16];
  struct neighbor *neighbor;
} person_t;

typedef struct neighbor {
  char name[16];
  int happiness;
  struct neighbor *next;
} neighbor_t;

static bool add_neighbors(person_t *attendees, size_t *len, char *person_name,
                          char *neighbor_name, int happiness) {
  // Find the index in locations which is either empty or equal to the given
  // source, whichever comes first.
  size_t idx = 0;
  do {
    if (!strcmp(attendees[idx].name, "")) {
      strcpy(attendees[idx].name, person_name);
      (*len)++;
      break;
    } else if (!strcmp(attendees[idx].name, person_name)) {
      break;
    }
    idx++;
  } while (idx <= *len);

  neighbor_t *neighbor = malloc(sizeof(neighbor_t));
  if (neighbor == NULL) {
    return false;
  }
  strcpy(neighbor->name, neighbor_name);
  neighbor->happiness = happiness;

  // Add the node to the head of linked list.
  neighbor->next = attendees[idx].neighbor;
  attendees[idx].neighbor = neighbor;
  return true;
}

static int happiness_between(person_t *from, person_t *to) {
  neighbor_t *n = from->neighbor;
  while (strcmp(n->name, to->name)) {
    n = n->next;
  }
  return n->happiness;
}

static int compute_happiness_change(person_t *attendees, size_t len) {
  int happiness = 0;
  happiness += happiness_between(&attendees[0], &attendees[len - 1]);
  happiness += happiness_between(&attendees[0], &attendees[1]);
  happiness += happiness_between(&attendees[len - 1], &attendees[len - 2]);
  happiness += happiness_between(&attendees[len - 1], &attendees[0]);
  for (size_t i = 1; i < len - 1; i++) {
    happiness += happiness_between(&attendees[i], &attendees[i - 1]);
    happiness += happiness_between(&attendees[i], &attendees[i + 1]);
  }
  return happiness;
}

static void free_attendees(person_t *attendees, size_t len) {
  for (size_t i = 0; i < len; i++) {
    neighbor_t *n = attendees[i].neighbor;
    while (n) {
      neighbor_t *temp = n;
      n = n->next;
      free(temp);
    }
  }
  free(attendees);
}

static void swap(person_t *p1, person_t *p2) {
  person_t temp = *p1;
  *p1 = *p2;
  *p2 = temp;
}

static void max_happiness_change(person_t *attendees, size_t len, size_t k,
                                 int *max) {
  if (k == 1) {
    int happiness_change = compute_happiness_change(attendees, len);
    if (happiness_change > *max) {
      *max = happiness_change;
    }
    return;
  }
  max_happiness_change(attendees, len, k - 1, max);
  for (size_t i = 0; i < k - 1; i++) {
    if (k % 2 == 0) {
      swap(attendees + i, attendees + k - 1);
    } else {
      swap(attendees, attendees + k - 1);
    }
    max_happiness_change(attendees, len, k - 1, max);
  }
}

int year2015_sol13(char *input) {
  char **lines = NULL;
  ssize_t line_cnt = readlines(&lines, input);
  if (line_cnt == -1) {
    perror(input);
    return EXIT_FAILURE;
  }

  size_t len = 0;  // number of attendees added
  person_t *attendees = calloc(BUFFER_SIZE, sizeof(person_t));
  if (attendees == NULL) {
    perror(NULL);
    return EXIT_FAILURE;
  }

  int retval = EXIT_FAILURE;
  char p1[16];
  char p2[16];
  char token[8];  // gain or lose
  int happiness;
  for (int i = 0; i < line_cnt; i++) {
    if (sscanf(lines[i],
               "%s would %s %d happiness units by sitting next to %[^.].", p1,
               token, &happiness, p2) != 4) {
      fprintf(stderr, "invalid line: %s\n", lines[i]);
      goto free_return;
    }
    if (!strcmp(token, "lose"))
      happiness = -happiness;
    if (!add_neighbors(attendees, &len, p1, p2, happiness)) {
      perror("add_neighbors");
      goto free_return;
    }
  }

  int max = 0;
  max_happiness_change(attendees, len, len, &max);
  printf("13.1: %d\n", max);

  // Add "Myself" will update the len variable, so let's use the actual value
  // in the for loop condition.
  size_t actual_len = len;
  for (size_t i = 0; i < actual_len; i++) {
    if (!add_neighbors(attendees, &len, attendees[i].name, "Myself", 0)) {
      perror("add_neighbors(Myself)");
      goto free_return;
    }
    if (!add_neighbors(attendees, &len, "Myself", attendees[i].name, 0)) {
      perror("add_neighbors(Myself)");
      goto free_return;
    }
  }

  max = 0;
  max_happiness_change(attendees, len, len, &max);
  printf("13.2: %d\n", max);
  retval = EXIT_SUCCESS;

free_return:
  free_attendees(attendees, len);
  return retval;
}
