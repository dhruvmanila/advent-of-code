#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../lib/read.h"

#define GRID_SIZE 100  // 100 x 100

typedef enum {
  OFF,
  ON,
} light_state_t;

static light_state_t *grid_new() {
  light_state_t *grid = calloc(GRID_SIZE * GRID_SIZE, sizeof(light_state_t));
  if (grid == NULL) {
    return NULL;
  }
  return grid;
}

static light_state_t grid_at(light_state_t *grid, int row, int col) {
  if (row < 0 || col < 0 || row >= GRID_SIZE || col >= GRID_SIZE) {
    return OFF;
  }
  return grid[row * GRID_SIZE + col];
}

static void grid_set(light_state_t *grid, int row, int col,
                     light_state_t state) {
  grid[row * GRID_SIZE + col] = state;
}

static int grid_neighbor_on_count(light_state_t *grid, int row, int col) {
  int count = 0;
  for (int dy = -1; dy <= 1; dy++) {
    for (int dx = -1; dx <= 1; dx++) {
      if (dx == 0 && dy == 0) {
        continue;
      }
      if (grid_at(grid, row + dy, col + dx)) {
        count++;
      }
    }
  }
  return count;
}

static light_state_t *grid_step(light_state_t *grid) {
  light_state_t *new_grid = grid_new();
  if (new_grid == NULL) {
    return NULL;
  }
  for (int r = 0; r < GRID_SIZE; r++) {
    for (int c = 0; c < GRID_SIZE; c++) {
      int on_count = grid_neighbor_on_count(grid, r, c);
      switch (grid_at(grid, r, c)) {
        case ON:
          grid_set(new_grid, r, c, (on_count == 2 || on_count == 3) ? ON : OFF);
          break;
        case OFF:
          grid_set(new_grid, r, c, (on_count == 3) ? ON : OFF);
          break;
      }
    }
  }
  free(grid);
  return new_grid;
}

static int grid_on_count(light_state_t *grid) {
  int count = 0;
  for (int i = 0; i < GRID_SIZE * GRID_SIZE; i++) {
    count += grid[i];
  }
  return count;
}

int year2015_sol18(char *input) {
  char **lines = NULL;
  ssize_t line_cnt = readlines(&lines, input);
  if (line_cnt == -1) {
    perror(input);
    return EXIT_FAILURE;
  }

  light_state_t *grid1 = grid_new();
  light_state_t *grid2 = grid_new();
  if (grid1 == NULL || grid2 == NULL) {
    perror(NULL);
    return EXIT_FAILURE;
  }

  for (int i = 0; i < GRID_SIZE; i++) {
    for (int j = 0; j < GRID_SIZE; j++) {
      grid_set(grid1, i, j, lines[i][j] == '#' ? ON : OFF);
      grid_set(grid2, i, j, lines[i][j] == '#' ? ON : OFF);
    }
  }

  for (int i = 100; i > 0; i--) {
    grid1 = grid_step(grid1);
    grid2 = grid_step(grid2);
    grid_set(grid2, 0, 0, ON);
    grid_set(grid2, 0, GRID_SIZE - 1, ON);
    grid_set(grid2, GRID_SIZE - 1, 0, ON);
    grid_set(grid2, GRID_SIZE - 1, GRID_SIZE - 1, ON);
    if (grid1 == NULL || grid2 == NULL) {
      perror(NULL);
      return EXIT_FAILURE;
    }
  }

  printf("18.1: %d\n18.2: %d\n", grid_on_count(grid1), grid_on_count(grid2));
  free(grid1);
  free(grid2);
  return EXIT_SUCCESS;
}
