#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "../lib/read.h"

#define MAX_TEASPOONS 100
#define TARGET_CALORIES 500

#define PRINT_INGREDIENT(i)                                                    \
  fprintf(stdout,                                                              \
          "ingredient{name:%s capacity:%d durability:%d flavor:%d texture:%d " \
          "calories:%d}\n",                                                    \
          (i).name, (i).capacity, (i).durability, (i).flavor, (i).texture,     \
          (i).calories)

typedef struct {
  char name[16];
  int capacity;
  int durability;
  int flavor;
  int texture;
  int calories;
} ingredient_t;

static ingredient_t cookie;

static void make_cookie(ingredient_t *ingredients, int proportions[4]) {
  cookie = (ingredient_t){0};
  for (int i = 0; i < 4; i++) {
    cookie.capacity += ingredients[i].capacity * proportions[i];
    cookie.durability += ingredients[i].durability * proportions[i];
    cookie.flavor += ingredients[i].flavor * proportions[i];
    cookie.texture += ingredients[i].texture * proportions[i];
    cookie.calories += ingredients[i].calories * proportions[i];
  }
}

static uint64_t score_cookie(void) {
  if (cookie.capacity < 0 || cookie.durability < 0 || cookie.flavor < 0 ||
      cookie.texture < 0) {
    return 0;
  }
  return cookie.capacity * cookie.durability * cookie.flavor * cookie.texture;
}

int year2015_sol15(char *input) {
  char **lines = NULL;
  ssize_t line_cnt = readlines(&lines, input);
  if (line_cnt == -1) {
    perror(input);
    return EXIT_FAILURE;
  }

  ingredient_t *ingredients = calloc(line_cnt, sizeof(ingredient_t));
  if (ingredients == NULL) {
    perror(NULL);
    return EXIT_FAILURE;
  }

  for (int i = 0; i < line_cnt; i++) {
    sscanf(lines[i],
           "%[^:]: capacity %d, durability %d, flavor %d, texture %d, "
           "calories %d",
           ingredients[i].name, &ingredients[i].capacity,
           &ingredients[i].durability, &ingredients[i].flavor,
           &ingredients[i].texture, &ingredients[i].calories);
  }

  uint64_t score;
  uint64_t maxscore1 = 0;
  uint64_t maxscore2 = 0;
  for (int a = 1; a < MAX_TEASPOONS; a++) {
    for (int b = 1; b < MAX_TEASPOONS - a; b++) {
      for (int c = 1; c < MAX_TEASPOONS - a - b; c++) {
        int d = MAX_TEASPOONS - a - b - c;
        make_cookie(ingredients, (int[4]){a, b, c, d});
        score = score_cookie();
        if (cookie.calories == TARGET_CALORIES && score > maxscore2) {
          maxscore2 = score;
        }
        if (score > maxscore1) {
          maxscore1 = score;
        }
      }
    }
  }

  free(ingredients);
  printf("15.1: %llu\n15.2: %llu\n", maxscore1, maxscore2);
  return EXIT_SUCCESS;
}
