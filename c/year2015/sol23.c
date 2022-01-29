#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../lib/read.h"

typedef enum {
  OP_HLF,
  OP_TPL,
  OP_INC,
  OP_JMP,
  OP_JIE,
  OP_JIO,
} opcode_t;

typedef enum {
  REG_A,
  REG_B,
} registers_t;

typedef struct {
  opcode_t op;
  registers_t reg;
  int offset;
} instruction_t;

static void execute(instruction_t *instructions, size_t len, int registers[2]) {
  size_t idx = 0;
  while (0 <= idx && idx < len) {
    instruction_t instruction = instructions[idx];
    switch (instruction.op) {
      case OP_HLF:
        registers[instruction.reg] /= 2;
        idx++;
        break;
      case OP_TPL:
        registers[instruction.reg] *= 3;
        idx++;
        break;
      case OP_INC:
        registers[instruction.reg]++;
        idx++;
        break;
      case OP_JMP:
        idx += instruction.offset;
        break;
      case OP_JIE:
        idx += registers[instruction.reg] % 2 == 0 ? instruction.offset : 1;
        break;
      case OP_JIO:
        idx += registers[instruction.reg] == 1 ? instruction.offset : 1;
        break;
    }
  }
}

int year2015_sol23(char *input) {
  char **lines = NULL;
  ssize_t line_cnt = readlines(&lines, input);
  if (line_cnt == -1) {
    perror(input);
    return EXIT_FAILURE;
  }

  instruction_t *instructions = calloc(line_cnt, sizeof(instruction_t));
  if (instructions == NULL) {
    perror(NULL);
    return EXIT_FAILURE;
  }

  for (int i = 0; i < line_cnt; i++) {
    char *opcode = strsep(&lines[i], " ");
    if (!strcmp(opcode, "hlf")) {
      instructions[i] = (instruction_t){
          .op = OP_HLF, .reg = *lines[i] == 'a' ? REG_A : REG_B};
    } else if (!strcmp(opcode, "tpl")) {
      instructions[i] = (instruction_t){
          .op = OP_TPL, .reg = *lines[i] == 'a' ? REG_A : REG_B};
    } else if (!strcmp(opcode, "inc")) {
      instructions[i] = (instruction_t){
          .op = OP_INC, .reg = *lines[i] == 'a' ? REG_A : REG_B};
    } else if (!strcmp(opcode, "jmp")) {
      instructions[i] = (instruction_t){.op = OP_JMP, .offset = atoi(lines[i])};
    } else if (!strcmp(opcode, "jie")) {
      instructions[i] = (instruction_t){
          .op = OP_JIE,
          .reg = (*strsep(&lines[i], ",") == 'a') ? REG_A : REG_B,
          .offset = atoi(lines[i])};
    } else if (!strcmp(opcode, "jio")) {
      instructions[i] = (instruction_t){
          .op = OP_JIO,
          .reg = (*strsep(&lines[i], ",") == 'a') ? REG_A : REG_B,
          .offset = atoi(lines[i])};
    } else {
      fprintf(stderr, "invalid opcode: %s\n", opcode);
    }
  }

  int registers[2] = {0, 0};  // REG_A (0), REG_B (1)
  execute(instructions, line_cnt, registers);
  printf("23.1: %d\n", registers[REG_B]);

  registers[REG_A] = 1;
  registers[REG_B] = 0;
  execute(instructions, line_cnt, registers);
  printf("23.2: %d\n", registers[REG_B]);

  free(instructions);
  return EXIT_SUCCESS;
}
