package util

// Reverse reverses the order of elements in the given slice in place.
func Reverse[T any](sl []T) {
	for i, j := 0, len(sl)-1; i < j; i, j = i+1, j-1 {
		sl[i], sl[j] = sl[j], sl[i]
	}
}

// MinMax returns the minimum and maximum value in the given slice of integers.
// This will panic if the slice is empty.
func MinMax(sl []int) (int, int) {
	if len(sl) == 0 {
		panic("util.MinMax: empty slice")
	}
	min, max := sl[0], sl[0]
	for _, val := range sl {
		min = Min(min, val)
		max = Max(max, val)
	}
	return min, max
}
