package util

import (
	"strconv"

	"golang.org/x/exp/constraints"
)

// Sum is used to add all the integers in a given array.
func Sum[T constraints.Integer | constraints.Float | constraints.Complex](arr []T) T {
	var total T
	for _, n := range arr {
		total += n
	}
	return total
}

// SumN is used to sum 1 to n integers.
func SumN[T constraints.Integer](n T) T {
	return n * (n + 1) / 2
}

// Max returns the larger of x or y.
func Max[T constraints.Integer | constraints.Float](x, y T) T {
	if x > y {
		return x
	}
	return y
}

// Min returns the smaller of x or y.
func Min[T constraints.Integer | constraints.Float](x, y T) T {
	if x < y {
		return x
	}
	return y
}

// Abs returns an absolute value of the given integer or floating-point number.
func Abs[T constraints.Integer | constraints.Float](n T) T {
	if n < 0 {
		return -n
	}
	return n
}

// Mod returns a % b, specifically the least positive remainder. This is
// different than the builtin % operator which returns the least negative
// remainder. This should only be used if either a or b is negative. Mod
// behaves the same as the builtin % operator when both a and b are positive.
func Mod[T constraints.Integer](a, b T) T {
	// https://stackoverflow.com/q/43018206
	return ((a % b) + b) % b
}

// Digits is used to iterate over each digit of the given number from left to
// right. This returns a channel from which you can only receive an integer one
// at a time and can be used in various ways like so:
//
//	for d := range Digits(123) {
//	  // do something with d
//	}
//
//	ch := Digits(123)
//	// ... other code
//	fmt.Println(<-ch)
//	// ... other code
//	fmt.Println(<-ch)
func Digits(n int) <-chan int {
	ns := strconv.Itoa(n)
	ch := make(chan int, len(ns))
	go func() {
		for _, d := range ns {
			ch <- int(d - '0')
		}
		close(ch)
	}()
	return ch
}

// Returns a number representing sign of n.
//
//   - 0 if the number is zero
//   - 1 if the number is positive
//   - -1 if the number is negative
func Signum[T constraints.Signed | constraints.Float](n T) T {
	switch {
	case n == 0:
		return 0
	case n > 0:
		return 1
	default:
		return -1
	}
}
