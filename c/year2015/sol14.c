#include <stdio.h>
#include <stdlib.h>

#include "../lib/read.h"

#define MAX_NAME_SIZE 8  // maximum number of characters for reindeer's name
#define TIME_LIMIT 2503  // time limit for the race

#define MAX(a, b) ((a) > (b) ? (a) : (b))

#define PRINT_REINDEER(r)                                                 \
  fprintf(stdout,                                                         \
          "reindeer{name:%s speed:%d flytime:%d resttime:%d distance:%d " \
          "points:%d state:%d}\n",                                        \
          (r).name, (r).speed, (r).flytime, (r).resttime, (r).distance,   \
          (r).points, (r).state)

typedef enum {
  RESTING,
  FLYING,
} state_t;

typedef struct {
  char name[MAX_NAME_SIZE];
  int speed;      // reindeer speed in km/s
  int flytime;    // flying time in seconds
  int resttime;   // rest time in seconds
  int distance;   // distance traveled so far
  int points;     // total points accumulated
  int remtime;    // time remaining until state change
  state_t state;  // current state (resting or flying)
} reindeer_t;

static void simulate(reindeer_t *rp, int until, int runners) {
  for (; until > 0; until--) {
    int maxdist = 0;  // maximum distance at time t
    for (int i = 0; i < runners; i++) {
      reindeer_t *r = rp + i;  // to update the struct in memory
      if (r->remtime == 0) {
        r->state = !r->state;  // only two possible states
        r->remtime = (r->state == RESTING) ? r->resttime : r->flytime;
      }
      if (r->state == FLYING) {
        r->distance += r->speed;
      }
      if (r->distance > maxdist) {
        maxdist = r->distance;
      }
      r->remtime--;
    }
    for (int i = 0; i < runners; i++) {
      reindeer_t *r = rp + i;
      r->points += r->distance == maxdist;
    }
  }
}

int year2015_sol14(char *input) {
  char **lines = NULL;
  ssize_t line_cnt = readlines(&lines, input);
  if (line_cnt == -1) {
    perror(input);
    return EXIT_FAILURE;
  }

  reindeer_t *rs = calloc(line_cnt, sizeof(reindeer_t));
  if (rs == NULL) {
    perror(NULL);
    return EXIT_FAILURE;
  }

  for (int i = 0; i < line_cnt; i++) {
    sscanf(
        lines[i],
        "%s can fly %d km/s for %d seconds, but then must rest for %d seconds.",
        rs[i].name, &rs[i].speed, &rs[i].flytime, &rs[i].resttime);
    rs[i].state = FLYING;  // initial state
    rs[i].remtime = rs[i].flytime;
  }

  simulate(rs, TIME_LIMIT, line_cnt);

  int maxdist = 0;
  int maxpoints = 0;
  for (int i = 0; i < line_cnt; i++) {
    maxdist = MAX(maxdist, rs[i].distance);
    maxpoints = MAX(maxpoints, rs[i].points);
  }

  free(rs);
  printf("12.1: %d\n12.2: %d\n", maxdist, maxpoints);
  return EXIT_SUCCESS;
}
