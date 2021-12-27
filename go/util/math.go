package util

import "strconv"

// Sum is used to add all the integers in a given array.
func Sum(arr []int) int {
	total := 0
	for _, n := range arr {
		total += n
	}
	return total
}

// SumN is used to sum 1 to n integers.
func SumN(n int) int {
	return n * (n + 1) / 2
}

// IntMax returns the larger of x or y integer.
func IntMax(x, y int) int {
	if x > y {
		return x
	}
	return y
}

// IntMin returns the smaller of x or y integer.
func IntMin(x, y int) int {
	if x < y {
		return x
	}
	return y
}

// AbsInt returns an absolute value of the given integer.
func AbsInt(n int) int {
	if n < 0 {
		return -n
	}
	return n
}

// Digits is used to iterate over each digit of the given number from left to
// right. This returns a channel from which you can only receive an integer one
// at a time and can be used in various ways like so:
//
//   for d := range Digits(123) {
//     // do something with d
//   }
//
//   ch := Digits(123)
//   // ... other code
//   fmt.Println(<-ch)
//   // ... other code
//   fmt.Println(<-ch)
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
