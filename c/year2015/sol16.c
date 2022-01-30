#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../lib/read.h"

#define MAX_AUNTS 500
#define MAX_COMPOUNDS_PER_AUNT 3

// Values on the ticker tape.
#define TAPE_CHILDREN 3
#define TAPE_CATS 7
#define TAPE_SAMOYEDS 2
#define TAPE_POMERANIANS 3
#define TAPE_AKITAS 0
#define TAPE_VIZSLAS 0
#define TAPE_GOLDFISH 5
#define TAPE_TREES 3
#define TAPE_CARS 2
#define TAPE_PERFUMES 1

#define PRINT_AUNT(idx, a)                                            \
  fprintf(stdout, "Sue %d: %s: %d, %s: %d, %s: %d\n", idx, a[0].name, \
          a[0].quantity, a[1].name, a[1].quantity, a[2].name, a[2].quantity)

typedef struct compound {
  char name[16];
  int quantity;
} compound_t;

static compound_t aunts[MAX_AUNTS][MAX_COMPOUNDS_PER_AUNT];

int year2015_sol16(char *input) {
  char **lines = NULL;
  ssize_t line_cnt = readlines(&lines, input);
  if (line_cnt == -1) {
    perror(input);
    return EXIT_FAILURE;
  }

  for (int i = 0; i < line_cnt; i++) {
    sscanf(lines[i], "Sue %*d: %[^:]: %d, %[^:]: %d, %[^:]: %d",
           aunts[i][0].name, &aunts[i][0].quantity, aunts[i][1].name,
           &aunts[i][1].quantity, aunts[i][2].name, &aunts[i][2].quantity);
  }

  int aunt1 = 0;
  int aunt2 = 0;
  for (int i = 0; !(aunt1 && aunt2) && i < MAX_AUNTS; i++) {
    bool match1 = true;
    bool match2 = true;
    for (int j = 0; j < MAX_COMPOUNDS_PER_AUNT; j++) {
      compound_t c = aunts[i][j];
      if (!strcmp(c.name, "children")) {
        match1 = match1 && c.quantity == TAPE_CHILDREN;
        match2 = match2 && c.quantity == TAPE_CHILDREN;
      }
      if (!strcmp(c.name, "cats")) {
        match1 = match1 && c.quantity == TAPE_CATS;
        match2 = match2 && c.quantity > TAPE_CATS;
      }
      if (!strcmp(c.name, "samoyeds")) {
        match1 = match1 && c.quantity == TAPE_SAMOYEDS;
        match2 = match2 && c.quantity == TAPE_SAMOYEDS;
      }
      if (!strcmp(c.name, "pomeranians")) {
        match1 = match1 && c.quantity == TAPE_POMERANIANS;
        match2 = match2 && c.quantity < TAPE_POMERANIANS;
      }
      if (!strcmp(c.name, "akitas")) {
        match1 = match1 && c.quantity == TAPE_AKITAS;
        match2 = match2 && c.quantity == TAPE_AKITAS;
      }
      if (!strcmp(c.name, "vizslas")) {
        match1 = match1 && c.quantity == TAPE_VIZSLAS;
        match2 = match2 && c.quantity == TAPE_VIZSLAS;
      }
      if (!strcmp(c.name, "goldfish")) {
        match1 = match1 && c.quantity == TAPE_GOLDFISH;
        match2 = match2 && c.quantity < TAPE_GOLDFISH;
      }
      if (!strcmp(c.name, "trees")) {
        match1 = match1 && c.quantity == TAPE_TREES;
        match2 = match2 && c.quantity > TAPE_TREES;
      }
      if (!strcmp(c.name, "cars")) {
        match1 = match1 && c.quantity == TAPE_CARS;
        match2 = match2 && c.quantity == TAPE_CARS;
      }
      if (!strcmp(c.name, "perfumes")) {
        match1 = match1 && c.quantity == TAPE_PERFUMES;
        match2 = match2 && c.quantity == TAPE_PERFUMES;
      }
    }
    if (match1) {
      aunt1 = i + 1;
    }
    if (match2) {
      aunt2 = i + 1;
    }
  }

  printf("16.1: %d\n16.2: %d\n", aunt1, aunt2);
  return EXIT_SUCCESS;
}
