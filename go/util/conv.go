package util

import (
	"strconv"
)

// Atoi is similar to strconv.Atoi except this function will panic if the
// conversion fails.
func Atoi(s string) int {
	i, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}
	return i
}

// Btoi is equivalent to util.ParseInt(s, 2, 0), converted to type int.
// The B is to represent the conversion from binary string to int.
func Btoi(s string) int {
	return int(ParseInt(s, 2, 0))
}

// ParseInt is similar to strconv.ParseInt except this function will panic if
// the parsing fails.
func ParseInt(s string, base int, bitSize int) int64 {
	i, err := strconv.ParseInt(s, base, bitSize)
	if err != nil {
		panic(err)
	}
	return i
}
