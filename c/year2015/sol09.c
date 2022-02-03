#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../lib/read.h"

// Size of the locations buffer such that it is equal to or greater than all the
// possible unique city names for both the test and actual puzzle input.
#define BUFFER_SIZE 8  // size should be in power of 2

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

// Add the given location pair to the locations list. If the source node does
// not exists, it will be allocated with the information, appended to locations
// and the len parameter will be updated accordingly.
//
// This function returns false in case of any memory allocation failure, true
// otherwise.
static bool add_location_pair(source_t *locations, size_t *len, char *source,
                              char *destination, int distance) {
  // Find the index in locations which is either empty or equal to the given
  // source, whichever comes first.
  size_t idx = 0;
  do {
    if (!strcmp(locations[idx].name, "")) {
      strcpy(locations[idx].name, source);
      (*len)++;
      break;
    } else if (!strcmp(locations[idx].name, source)) {
      break;
    }
    idx++;
  } while (idx <= *len);

  destination_t *dest = malloc(sizeof(destination_t));
  if (dest == NULL) {
    return false;
  }
  strcpy(dest->name, destination);
  dest->distance = distance;

  // Add the node to the head of linked list.
  dest->next = locations[idx].dest;
  locations[idx].dest = dest;
  return true;
}

// Return the distance between from and to.
static int distance_between(source_t *from, source_t *to) {
  destination_t *dest = from->dest;
  while (strcmp(dest->name, to->name)) {
    dest = dest->next;
  }
  return dest->distance;
}

// Return the total distance between all the given locations in order.
static int total_distance(source_t *locations, size_t len) {
  int distance = 0;
  for (size_t i = 0; i < len - 1; i++) {
    distance += distance_between(&locations[i], &locations[i + 1]);
  }
  return distance;
}

static void free_locations(source_t *locations, size_t len) {
  for (size_t i = 0; i < len; i++) {
    destination_t *dest = locations[i].dest;
    while (dest) {
      destination_t *temp = dest;
      dest = dest->next;
      free(temp);
    }
  }
  free(locations);
}

static void swap(source_t *s1, source_t *s2) {
  source_t temp = *s1;
  *s1 = *s2;
  *s2 = temp;
}

// Compute the minimum and maximum distance for path including all the given
// locations once and update the min and max parameter accordingly. This will
// mutate the locations array as it will arrange the locations to account for
// all permutations.
static void minmax(source_t *locations, size_t len, size_t k, int *min,
                   int *max) {
  if (k == 1) {
    int distance = total_distance(locations, len);
    if (distance < *min) {
      *min = distance;
    }
    if (distance > *max) {
      *max = distance;
    }
    return;
  }

  minmax(locations, len, k - 1, min, max);
  for (size_t i = 0; i < k - 1; i++) {
    if (k % 2 == 0) {
      swap(locations + i, locations + k - 1);
    } else {
      swap(locations, locations + k - 1);
    }
    minmax(locations, len, k - 1, min, max);
  }
}

int year2015_sol09(char *input) {
  char **lines = NULL;
  ssize_t line_cnt = readlines(&lines, input);
  if (line_cnt == -1) {
    perror(input);
    return EXIT_FAILURE;
  }

  size_t nlocations = 0;  // number of source locations added
  source_t *locations = calloc(BUFFER_SIZE, sizeof(source_t));
  if (locations == NULL) {
    perror(NULL);
    return EXIT_FAILURE;
  }

  int retval = EXIT_FAILURE;
  char from[16];  // source city name
  char to[16];    // destination city name
  int distance;   // distance between source and destination
  for (int i = 0; i < line_cnt; i++) {
    if (sscanf(lines[i], "%s to %s = %d", from, to, &distance) != 3) {
      fprintf(stderr, "invalid line: '%s'\n", lines[i]);
      goto free_return;
    }
    if (!add_location_pair(locations, &nlocations, from, to, distance)) {
      perror("add_location_pair");
      goto free_return;
    }
    if (!add_location_pair(locations, &nlocations, to, from, distance)) {
      perror("add_location_pair");
      goto free_return;
    }
  }

  int min = INT32_MAX;
  int max = 0;
  minmax(locations, nlocations, nlocations, &min, &max);

  printf("9.1: %d\n9.2: %d\n", min, max);
  retval = EXIT_SUCCESS;

free_return:
  free_locations(locations, nlocations);
  return retval;
}
