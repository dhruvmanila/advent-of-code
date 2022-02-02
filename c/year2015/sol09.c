#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../lib/read.h"

#define MAX_LOCATIONS 8  // possible unique city names for the puzzle

#define PRINT_SOURCE(location)                          \
  do {                                                  \
    printf("%s", location.name);                        \
    destination_t *dest = location.dest;                \
    while (dest) {                                      \
      printf(" -> %s[%d]", dest->name, dest->distance); \
      dest = dest->next;                                \
    }                                                   \
    printf("\n");                                       \
  } while (0)

typedef struct {
  char name[16];             // source city name
  struct destination *dest;  // head of linked list for all the destinations
} source_t;

typedef struct destination {
  char name[16];             // destination city name
  int distance;              // distance from source to destination
  struct destination *next;  // next node in all destinations linked list
} destination_t;

static source_t locations[MAX_LOCATIONS];

static bool add_location_pair(char *source, char *destination, int distance) {
  destination_t *dest = malloc(sizeof(destination_t));
  if (dest == NULL) {
    return false;
  }
  strcpy(dest->name, destination);
  dest->distance = distance;

  int idx = 0;
  for (; idx < MAX_LOCATIONS; idx++) {
    if (!strcmp(locations[idx].name, "")) {
      strcpy(locations[idx].name, source);
      locations[idx].dest = dest;
      return true;
    } else if (!strcmp(locations[idx].name, source)) {
      break;
    }
  }

  destination_t *current = locations[idx].dest;
  while (current->next) {
    current = current->next;
  }
  current->next = dest;
  return true;
}

static int distance_between(source_t *from, source_t *to) {
  destination_t *dest = from->dest;
  while (strcmp(dest->name, to->name)) {
    dest = dest->next;
  }
  return dest->distance;
}

static int total_distance(source_t *source) {
  int distance = 0;
  for (int i = 0; i < MAX_LOCATIONS - 1; i++) {
    distance += distance_between(&source[i], &source[i + 1]);
  }
  return distance;
}

static void free_locations() {
  for (int i = 0; i < MAX_LOCATIONS; i++) {
    destination_t *dest = locations[i].dest;
    while (dest) {
      destination_t *temp = dest;
      dest = dest->next;
      free(temp);
    }
  }
}

static void swap(source_t *s1, source_t *s2) {
  source_t temp = *s1;
  *s1 = *s2;
  *s2 = temp;
}

static void minmax(source_t *source, size_t k, int *min, int *max) {
  if (k == 1) {
    int distance = total_distance(source);
    if (distance < *min) {
      *min = distance;
    }
    if (distance > *max) {
      *max = distance;
    }
    return;
  }

  minmax(source, k - 1, min, max);
  for (size_t i = 0; i < k - 1; i++) {
    if (k % 2 == 0) {
      swap(source + i, source + k - 1);
    } else {
      swap(source, source + k - 1);
    }
    minmax(source, k - 1, min, max);
  }
}

int year2015_sol09(char *input) {
  char **lines = NULL;
  ssize_t line_cnt = readlines(&lines, input);
  if (line_cnt == -1) {
    perror(input);
    return EXIT_FAILURE;
  }

  int retval = EXIT_FAILURE;
  char from[16];
  char to[16];
  int distance;
  for (int i = 0; i < line_cnt; i++) {
    if (sscanf(lines[i], "%16s to %16s = %d", from, to, &distance) != 3) {
      fprintf(stderr, "invalid line: %s\n", lines[i]);
      goto free_return;
    }
    if (!add_location_pair(from, to, distance)) {
      perror("add_location_pair");
      goto free_return;
    }
    if (!add_location_pair(to, from, distance)) {
      perror("add_location_pair");
      goto free_return;
    }
  }

  int min = INT32_MAX;
  int max = 0;
  minmax(locations, MAX_LOCATIONS, &min, &max);

  printf("9.1: %d\n9.2: %d\n", min, max);
  retval = EXIT_SUCCESS;

free_return:
  free_locations();
  return retval;
}
