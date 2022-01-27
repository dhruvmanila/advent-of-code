#include <cjson/cJSON.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../lib/read.h"

// Refer to https://stackoverflow.com/a/1067238 for `do ... while(0)` pattern
// in multiline macro.

#define JSON_PRINT(element)                            \
  do {                                                 \
    char *str = cJSON_Print(element);                  \
    if (str == NULL) {                                 \
      fputs("failed to print JSON element\n", stderr); \
    }                                                  \
    puts(str);                                         \
    free(str);                                         \
  } while (0)

int sum(cJSON *json, const char *skip) {
  int total = 0;
  if (cJSON_IsNumber(json)) {
    return json->valueint;
  } else if (cJSON_IsArray(json)) {
    cJSON *element = NULL;
    cJSON_ArrayForEach(element, json) {
      total += sum(element, skip);
    }
  } else if (cJSON_IsObject(json)) {
    int subtotal = 0;
    cJSON *element = NULL;
    cJSON_ArrayForEach(element, json) {
      if (skip && cJSON_IsString(element) &&
          !strcmp(element->valuestring, skip)) {
        goto skip_object;
      }
      subtotal += sum(element, skip);
    }
    total += subtotal;
  }
skip_object:
  return total;
}

int year2015_sol12(char *input) {
  char **lines = NULL;
  ssize_t line_cnt = readlines(&lines, input);
  if (line_cnt == -1) {
    perror(input);
    return EXIT_FAILURE;
  }

  int retval = EXIT_FAILURE;
  cJSON *json = cJSON_Parse(lines[0]);
  if (json == NULL) {
    const char *errp = cJSON_GetErrorPtr();
    if (errp != NULL) {
      fprintf(stderr, "error parsing JSON: %s\n", errp);
    }
    goto exit;
  }

  printf("12.1: %d\n12.2: %d\n", sum(json, NULL), sum(json, "red"));
  retval = EXIT_SUCCESS;

exit:
  cJSON_Delete(json);
  return retval;
}
