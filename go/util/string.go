package util

import (
	"sort"
)

// SortString is used to sort the individual characters in the given string.
func SortString(s string) string {
	ss := []rune(s)
	sort.Slice(ss, func(i, j int) bool {
		return ss[i] < ss[j]
	})
	return string(ss)
}
