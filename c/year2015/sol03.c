#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "../lib/read.h"

typedef struct house {
  int64_t x;
  int64_t y;
  struct house *next;
} house_t;

static house_t *head = NULL;  // pointer to the first element of linked list

// house_new is used to allocate memory for a house_t struct and initialize it
// with the given x and y values. It will initalize the next pointer to be NULL.
// It returns a pointer to house_t or NULL, in case malloc() failed to allocate
// memory.
static house_t *house_new(int64_t x, int64_t y) {
  house_t *house = malloc(sizeof(house_t));
  if (house == NULL) {
    return NULL;
  }
  house->x = x;
  house->y = y;
  house->next = NULL;
  return house;
}

// house_find returns the true if there's a house containing the given value x
// and y, false otherwise.
static bool house_find(house_t *head, int64_t x, int64_t y) {
  house_t *current = head;
  while (current != NULL) {
    if (current->x == x && current->y == y) {
      return true;
    }
    current = current->next;
  }
  return false;
}

// house_add will add a house_t at given x and y position to the front of
// the head pointer. It returns 1 if successful, 0 if the item already exists
// and -1 if there were any errors during memory allocation.
static int8_t house_add(house_t **head, int64_t x, int64_t y) {
  bool house = house_find(*head, x, y);
  if (house) {
    return 0;
  }

  house_t *to_insert = house_new(x, y);
  if (to_insert == NULL) {  // malloc error
    return -1;
  }

  to_insert->next = *head;
  *head = to_insert;
  return 1;
}

// house_free frees up all the memory allocated.
static void house_free(void) {
  while (head != NULL) {
    house_t *tmp = head;
    head = head->next;
    free(tmp);
  }
}

static ssize_t presents_by_santa(char *moves) {
  int64_t pos[2] = {0, 0};  // x and y position of santa
  ssize_t result = -1;      // function result value
  ssize_t count = 0;        // number of houses the presents were delivered

  int8_t res = house_add(&head, pos[0], pos[1]);
  if (res == -1) {
    goto done;
  }
  count += res;

  for (char *ch = moves; *ch != '\0'; ch++) {
    switch (*ch) {
      case '<':
        pos[0]--;
        break;
      case '^':
        pos[1]++;
        break;
      case '>':
        pos[0]++;
        break;
      case 'v':
        pos[1]--;
        break;
      default:
        fprintf(stderr, "aoc: invalid character: %c\n", *ch);
        goto done;
    }
    int8_t res = house_add(&head, pos[0], pos[1]);
    if (res == -1) {
      goto done;
    }
    count += res;
  }
  result = count;

done:
  house_free();
  return result;
}

static ssize_t presents_by_robo_and_santa(char *moves) {
  int64_t pos[2][2] = {{0, 0},   // x and y position for santa
                       {0, 0}};  // x and y position for robot-santa
  uint8_t turn = 0;     // current turn for moving (0: santa, 1: robot-santa)
  ssize_t result = -1;  // function result value
  ssize_t count = 0;    // number of house presents were delivered

  int8_t res = house_add(&head, pos[turn][0], pos[turn][1]);
  if (res == -1) {
    goto done;
  }
  count += res;

  for (char *ch = moves; *ch != '\0'; ch++) {
    switch (*ch) {
      case '<':
        pos[turn][0]--;
        break;
      case '^':
        pos[turn][1]++;
        break;
      case '>':
        pos[turn][0]++;
        break;
      case 'v':
        pos[turn][1]--;
        break;
      default:
        fprintf(stderr, "aoc: invalid character: %c\n", *ch);
        goto done;
    }
    int8_t res = house_add(&head, pos[turn][0], pos[turn][1]);
    if (res == -1) {
      goto done;
    }
    count += res;
    turn = turn == 0 ? 1 : 0;
  }
  result = count;

done:
  house_free();
  return result;
}

int year2015_sol03(char *input) {
  char **lines = NULL;
  ssize_t linelen = readlines(&lines, input);
  if (linelen == -1) {
    perror(input);
    return EXIT_FAILURE;
  }
  char *line = lines[0];  // one line file

  ssize_t count1 = presents_by_santa(line);
  if (count1 == -1) {
    return EXIT_FAILURE;
  }

  ssize_t count2 = presents_by_robo_and_santa(line);
  if (count2 == -1) {
    return EXIT_FAILURE;
  }

  printf("3.1: %zu\n3.2: %zd\n", count1, count2);
  return EXIT_SUCCESS;
}
