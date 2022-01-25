#include <assert.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../lib/read.h"

typedef enum {
  OP_MOV,     // direct assignment; signal is provided to the wire
  OP_AND,     // bitwise AND
  OP_OR,      // bitwise OR
  OP_NOT,     // bitwise complement
  OP_LSHIFT,  // bitwise left-shift
  OP_RSHIFT,  // bitwise right-shift
} op_t;

typedef enum {
  NAME,
  SIGNAL,
} datatype_t;

typedef struct {
  datatype_t dtype;  // type of data in the struct
  union {
    char *name;       // wire name
    uint16_t signal;  // wire signal
  } data;
} data_t;

typedef struct wire {
  op_t op;          // operator
  char *out;        // out wire name
  data_t *in[2];    // in wire (x2) data (name or signal)
  uint8_t shift;    // shift value for LSHIFT and RSHIFT
  uint16_t signal;  // out wire signal
  bool done;        // out signal calculated
} wire_t;

wire_t *wires[702] = {0};  // 27 * 26

uint16_t wire_hash(const char *name) {
  uint16_t hash;
  switch (strlen(name)) {
    case 1:
      hash = *name;
      break;
    case 2:
      hash = 26 * (name[0] - 96) + name[1];
      break;
  }
  return hash - 97;
}

void wire_add(wire_t *w) {
  uint16_t hash = wire_hash(w->out);
  wires[hash] = w;
}

wire_t *wire_get(const char *name) {
  uint16_t hash = wire_hash(name);
  return wires[hash];
}

void wire_free() {
  for (int i = 0; i < 702; i++) {
    wire_t *w = wires[i];
    if (w) {
      for (int i = 0; i < 2; i++) {
        if (w->in[i]) {
          free(w->in[i]);
        }
      }
      free(w);
    }
  }
}

data_t *resolve_data(char *data) {
  data_t *d = malloc(sizeof(data_t));
  assert(d);
  if (isdigit(*data)) {
    d->dtype = SIGNAL;
    d->data.signal = atoi(data);
  } else {
    d->dtype = NAME;
    d->data.name = data;
  }
  return d;
}

bool parse_wire(char *line) {
  size_t nfields = 0;
  char *fields[5];  // max words in a line separated by a space
  while (line) {
    fields[nfields++] = strsep(&line, " ");
  }

  wire_t *w = malloc(sizeof(wire_t));
  if (w == NULL) {
    return false;
  }
  w->out = fields[nfields - 1];
  w->done = false;

  switch (nfields) {
    case 3:
      w->op = OP_MOV;
      w->in[0] = resolve_data(fields[0]);
      break;
    case 4:
      w->op = OP_NOT;
      w->in[0] = resolve_data(fields[1]);
      break;
    case 5:
      if (!strcmp(fields[1], "AND")) {
        w->op = OP_AND;
        w->in[0] = resolve_data(fields[0]);
        w->in[1] = resolve_data(fields[2]);
      } else if (!strcmp(fields[1], "OR")) {
        w->op = OP_OR;
        w->in[0] = resolve_data(fields[0]);
        w->in[1] = resolve_data(fields[2]);
      } else if (!strcmp(fields[1], "LSHIFT")) {
        w->op = OP_LSHIFT;
        w->in[0] = resolve_data(fields[0]);
        w->shift = atoi(fields[2]);
      } else if (!strcmp(fields[1], "RSHIFT")) {
        w->op = OP_RSHIFT;
        w->in[0] = resolve_data(fields[0]);
        w->shift = atoi(fields[2]);
      } else {
        fprintf(stderr, "parse error: invalid operator: %s\n", fields[1]);
        return false;
      }
      break;
    default:
      fprintf(stderr, "parse error: invalid expression\n");
      return false;
  }

  wire_add(w);
  return true;
}

uint16_t wire_signal(char *name) {
  wire_t *w = wire_get(name);
  if (w->done) {
    return w->signal;
  }

  uint16_t signals[2];
  data_t *data;
  for (int i = 0; i < 2; i++) {
    data = w->in[i];
    if (data != NULL) {
      switch (data->dtype) {
        case NAME:
          signals[i] = wire_signal(data->data.name);
          break;
        case SIGNAL:
          signals[i] = data->data.signal;
          break;
      }
    }
  }

  switch (w->op) {
    case OP_MOV:
      w->signal = signals[0];
      break;
    case OP_AND:
      w->signal = signals[0] & signals[1];
      break;
    case OP_OR:
      w->signal = signals[0] | signals[1];
      break;
    case OP_NOT:
      w->signal = ~signals[0];
      break;
    case OP_LSHIFT:
      w->signal = signals[0] << w->shift;
      break;
    case OP_RSHIFT:
      w->signal = signals[0] >> w->shift;
      break;
  }

  w->done = true;
  return w->signal;
}

int year2015_sol07(char *input) {
  char **lines = NULL;
  ssize_t line_cnt = readlines(&lines, input);
  if (line_cnt == -1) {
    perror(input);
    return EXIT_FAILURE;
  }

  for (int i = 0; i < line_cnt; i++) {
    if (!parse_wire(lines[i])) {
      perror(NULL);
      return EXIT_FAILURE;
    }
  }

  uint16_t a_signal = wire_signal("a");
  printf("6.1: %d\n", a_signal);

  for (int i = 0; i < 702; i++) {
    if (wires[i]) {
      wires[i]->done = false;
    }
  }
  wire_get("b")->signal = a_signal;
  wire_get("b")->done = true;
  a_signal = wire_signal("a");
  printf("6.2: %d\n", a_signal);

  wire_free();
  return EXIT_SUCCESS;
}
