#include <math.h>
#include <stdio.h>
#include <stdlib.h>

static const int input = 29000000;

static int presents1(int dividend) {
  int sum = 1 + dividend;
  for (int divisor = 2; divisor <= sqrt(dividend); divisor++) {
    if (dividend % divisor == 0) {
      int quotient = dividend / divisor;
      sum += divisor + (divisor != quotient ? quotient : 0);
    }
  }
  return sum * 10;
}

static int presents2(int dividend) {
  int sum = 0;
  for (int divisor = 1; divisor <= sqrt(dividend); divisor++) {
    if (dividend % divisor == 0) {
      int quotient = dividend / divisor;
      if (quotient <= 50) {
        sum += 11 * divisor;
      }
      if (quotient != divisor && divisor <= 50) {
        sum += 11 * quotient;
      }
    }
  }
  return sum;
}

int year2015_sol20(void) {
  int house1 = 0, house2 = 0;
  for (int i = 1; !(house1 && house2); i++) {
    if (!house1 && presents1(i) > input) {
      house1 = i;
    }
    if (!house2 && presents2(i) > input) {
      house2 = i;
    }
  }

  printf("20.1: %d\n20.2: %d\n", house1, house2);
  return EXIT_SUCCESS;
}
