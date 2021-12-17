package util

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

// MinMax is used to find the minimum and maximum value in the given array of
// integers.
func MinMax(arr []int) (int, int) {
	min, max := arr[0], arr[0]
	for _, val := range arr {
		min = IntMin(min, val)
		max = IntMax(max, val)
	}
	return min, max
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
