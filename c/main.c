#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>

#include "./year2015/solutions.h"

int main(int argc, char** argv) {
  int ch, day, year;
  bool tflag = false;

  time_t t = time(NULL);
  struct tm* timeinfo = localtime(&t);
  year = timeinfo->tm_year + 1900;  // tm_year is (year - 1900)
  day = timeinfo->tm_mday;
  if (timeinfo->tm_mon != 11) {
    year--;
    day = 25;
  } else if (day > 25) {
    day = 25;
  }

  while ((ch = getopt(argc, argv, ":y:d:th")) != -1) {
    switch (ch) {
      case 'y': {
        char* endptr;
        errno = 0;
        year = (int)strtol(optarg, &endptr, 10);
        if (endptr == optarg || *endptr != '\0' || errno == ERANGE) {
          fprintf(stderr, "aoc: invalid year: %s\n", optarg);
          return EXIT_FAILURE;
        }
        break;
      }
      case 'd': {
        char* endptr;
        errno = 0;
        day = (int)strtol(optarg, &endptr, 10);
        if (endptr == optarg || *endptr != '\0' || errno == ERANGE) {
          fprintf(stderr, "aoc: invalid day: %s\n", optarg);
          return EXIT_FAILURE;
        }
        break;
      }
      case 't':
        tflag = true;
        break;
      case ':':
        fprintf(stderr, "aoc: option '-%c' requires an argument\n", optopt);
        return EXIT_FAILURE;
      case '?':
        fprintf(stderr, "aoc: unknown flag: %c\n", optopt);
        return EXIT_FAILURE;
      case 'h':
      default:
        fprintf(stderr,
                "Usage: %s [-y <year>] [-d <day>] [-t]\n"
                "\n"
                "Options:\n"
                "  -d <day>   run solution for given day (default: %d)\n"
                "  -y <year>  run solution for given year (default: %d)\n"
                "  -t         run the test input instead\n",
                argv[0], day, year);
        return EXIT_SUCCESS;
    }
  }

  char fname[30];
  sprintf(fname, "./year%d/input%s/%02d.txt", year, (tflag == 1) ? "/test" : "",
          day);

  switch (year) {
    case 2015:
      switch (day) {
        case 1:
          return year2015_sol01(fname);
        case 2:
          return year2015_sol02(fname);
        case 3:
          return year2015_sol03(fname);
        case 4:
          return year2015_sol04();
        case 5:
          return year2015_sol05(fname);
        case 6:
          return year2015_sol06(fname);
        case 7:
          return year2015_sol07(fname);
      }
      break;
  }

  fprintf(stderr, "aoc: year %d: day %d: unsolved\n", year, day);
  return EXIT_FAILURE;
}
