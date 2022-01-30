#include <assert.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../lib/read.h"

// Maximum number of out wires possible when using a maximum of two letters to
// name them: 26 letters plus all combinations of them (26 * 26).
#define MAX_WIRES 702  // 27 * 26

typedef enum {
  OP_MOV,     // direct assignment; signal is provided to the wire
  OP_AND,     // bitwise AND
  OP_OR,      // bitwise OR
  OP_NOT,     // bitwise complement
  OP_LSHIFT,  // bitwise left-shift
  OP_RSHIFT,  // bitwise right-shift
} op_t;

typedef enum {
  NAME,    // wire name
  SIGNAL,  // wire signal
} wire_data_type_t;

typedef struct {
  wire_data_type_t type;  // type of data
  union {
    char *name;       // wire name
    uint16_t signal;  // wire signal
  } data;
} wire_data_t;

typedef struct wire {
  op_t op;             // operator
  char *out;           // out wire name
  wire_data_t *in[2];  // in wire (x2) data (name or signal)
  uint8_t shift;       // shift value for LSHIFT and RSHIFT
  uint16_t signal;     // out wire signal
  bool done;           // out signal calculated
} wire_t;

// Initialize MAX_WIRES number of wires in memory with all its field set to
// the zero values of the respective type.
static wire_t wires[MAX_WIRES];

// wire_hash computes the hash of the corresponding name. The computed hash is
// basically the index of the respective wire_t object in the wires array. The
// value calculated is based on the alphatical order of the name such that
// 'a' = 0, 'b' = 1, ..., 'z' = 25, 'aa' = 26, 'ab' = 27, ..., 'az' = 51,
// 'ba' = 52, ..., 'zz' = 701.
static uint16_t wire_hash(const char *name) {
  uint16_t hash;
  switch (strlen(name)) {
    case 1:
      hash = *name;  // ASCII value of the character
      break;
    case 2:
      hash = 26 * (name[0] - 96) + name[1];
      break;
  }
  // Resolve the value to start at 0 as 'a' is 97 in ASCII.
  return hash - 97;
}

// Return the wire at index position corresponding to the given out wire name.
// This returns a pointer to the wire object so that changes made is reflected
// in the array.
static wire_t *wire_get(const char *name) {
  return &wires[wire_hash(name)];
}

// Free all the resources allocated during the runtime.
static void wire_free() {
  for (int i = 0; i < MAX_WIRES; i++) {
    wire_t *w = &wires[i];
    for (int i = 0; i < 2; i++) {
      if (w->in[i]) {
        free(w->in[i]);
      }
    }
  }
}

// Resolve the given data which is either a wire name or wire signal to the
// internal representation of union of those data types. This will allocate
// the memory dynamically and use assertion to check for memory failure.
static wire_data_t *resolve_data(char *data) {
  wire_data_t *d = malloc(sizeof(wire_data_t));
  assert(d);
  if (isdigit(*data)) {
    d->type = SIGNAL;
    d->data.signal = atoi(data);
  } else {
    d->type = NAME;
    d->data.name = data;
  }
  return d;
}

static bool parse_wire(char *line) {
  size_t nfields = 0;
  char *fields[5];  // max words in a line separated by a space
  while (line) {
    fields[nfields++] = strsep(&line, " ");
  }

  wire_t *w = wire_get(fields[nfields - 1]);
  w->out = fields[nfields - 1];

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

  return true;
}

static uint16_t wire_signal(char *name) {
  wire_t *w = wire_get(name);
  if (w->done) {
    return w->signal;
  }

  uint16_t signals[2];
  wire_data_t *data;
  for (int i = 0; i < 2; i++) {
    data = w->in[i];
    if (data != NULL) {
      switch (data->type) {
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
      wire_free();
      perror(NULL);
      return EXIT_FAILURE;
    }
  }

  uint16_t a_signal = wire_signal("a");
  printf("6.1: %d\n", a_signal);

  // Reset all the wires and set the out signal of wire A to wire B.
  for (int i = 0; i < MAX_WIRES; i++) {
    wires[i].done = false;
  }
  wire_get("b")->signal = a_signal;
  wire_get("b")->done = true;

  a_signal = wire_signal("a");
  printf("6.2: %d\n", a_signal);

  wire_free();
  return EXIT_SUCCESS;
}
