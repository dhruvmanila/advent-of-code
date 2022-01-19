#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>

#include "./year2015/solutions.h"

int main(int argc, char *argv[]) {
  int ch, day, year;
  bool tflag = false;

  time_t t = time(NULL);
  struct tm *timeinfo = localtime(&t);
  year = timeinfo->tm_year + 1900; // tm_year is (year - 1900)
  day = timeinfo->tm_mday;
  if (timeinfo->tm_mon != 11) {
    year--;
    day = 25;
  } else if (day > 25) {
    day = 25;
  }

  while ((ch = getopt(argc, argv, ":y:d:th")) != -1) {
    char *endptr;
    switch (ch) {
    case 'y':
      errno = 0;
      year = (int)strtol(optarg, &endptr, 10);
      if (endptr == optarg || *endptr != '\0' || errno == ERANGE) {
        fprintf(stderr, "aoc: invalid year: %s\n", optarg);
        return 1;
      }
      break;
    case 'd':
      errno = 0;
      day = (int)strtol(optarg, &endptr, 10);
      if (endptr == optarg || *endptr != '\0' || errno == ERANGE) {
        fprintf(stderr, "aoc: invalid day: %s\n", optarg);
        return 1;
      }
      break;
    case 't':
      tflag = true;
      break;
    case ':':
      fprintf(stderr, "aoc: option '-%c' requires an argument\n", optopt);
      return 1;
    case '?':
      fprintf(stderr, "aoc: unknown flag: %c\n", optopt);
      return 1;
    case 'h':
    default:
      fprintf(stderr, "Usage: %s [-y <year>] [-d <day>] [-t]\n", argv[0]);
      fputs("\nOptions:\n", stderr);
      fprintf(stderr, "  -d <day>   run solution for given day (default: %d)\n", day);
      fprintf(stderr, "  -y <year>  run solution for given year (default: %d)\n", year);
      fputs("  -t         run the test input instead\n", stderr);
      return 0;
    }
  }

  char fname[30];
  sprintf(fname, "./year%d/input%s/%02d.txt", year, (tflag == 1) ? "/test" : "", day);

  switch (year) {
  case 2015:
    switch (day) {
    case 1: year2015_sol01(fname); break;
    default:
      fprintf(stderr, "aoc: year %d: day %d: unsolved\n", year, day);
      return 1;
    }
    break;
  default:
    fprintf(stderr, "aoc: year %d: day %d: unsolved\n", year, day);
    return 1;
  }

  return 0;
}
