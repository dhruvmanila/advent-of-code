#include <errno.h>
#include <stdbool.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>

#include "./year2015/solutions.h"

int main(int argc, char** argv) {
  int ch, day, year;

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

  while ((ch = getopt(argc, argv, ":y:d:h")) != -1) {
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
      case ':':
        fprintf(stderr, "aoc: option '-%c' requires an argument\n", optopt);
        return EXIT_FAILURE;
      case '?':
        fprintf(stderr, "aoc: unknown flag: %c\n", optopt);
        return EXIT_FAILURE;
      case 'h':
      default:
        fprintf(stdout,
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

  const char* home = getenv("HOME");
  if (!home) {
      fprintf(stderr, "aoc: HOME environment variable not set\n");
      return EXIT_FAILURE;
  }

  char fname[128];
  snprintf(fname, sizeof(fname), "%s/.cache/aoc/%d/%d.txt", home, year, day);

  struct stat file_stat;
  if (stat(fname, &file_stat) != 0) {
      if (errno == ENOENT) {
          fprintf(stderr, "aoc: input file not found at: %s\n", fname);
          fprintf(stderr, "aoc: download the input file from https://adventofcode.com/%d/day/%d/input\n", year, day);
          return EXIT_FAILURE;
      } else {
          fprintf(stderr, "aoc: %s: %s\n", fname, strerror(errno));
          return EXIT_FAILURE;
      }
  }

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
        case 8:
          return year2015_sol08(fname);
        case 9:
          return year2015_sol09(fname);
        case 10:
          return year2015_sol10();
        case 11:
          return year2015_sol11();
        case 12:
          return year2015_sol12(fname);
        case 13:
          return year2015_sol13(fname);
        case 14:
          return year2015_sol14(fname);
        case 15:
          return year2015_sol15(fname);
        case 16:
          return year2015_sol16(fname);
        case 17:
          return year2015_sol17(fname);
        case 18:
          return year2015_sol18(fname);
        case 20:
          return year2015_sol20();
        case 21:
          return year2015_sol21();
        case 23:
          return year2015_sol23(fname);
      }
      break;
  }

  fprintf(stderr, "aoc: year %d: day %d: unsolved\n", year, day);
  return EXIT_FAILURE;
}
