package util

import (
	"strconv"
)

// MustAtoi is like strconv.Atoi but panics if the conversion fails.
func MustAtoi(s string) int {
	i, err := strconv.Atoi(s)
	if err != nil {
		panic("util.MustAtoi: " + err.Error())
	}
	return i
}

// MustBtoi is equivalent to util.MustParseInt(s, 2, 0), converted to type int.
func MustBtoi(s string) int {
	return int(MustParseInt(s, 2, 0))
}

// MustParseInt is like strconv.ParseInt but panics if the parsing fails.
func MustParseInt(s string, base int, bitSize int) int64 {
	i, err := strconv.ParseInt(s, base, bitSize)
	if err != nil {
		panic("util.MustParseInt: " + err.Error())
	}
	return i
}
