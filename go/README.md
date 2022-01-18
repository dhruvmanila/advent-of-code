# Advent of Code (Go)

Advent of Code solutions in the Go Programming Language.

## `aoc` CLI tool

The project provides a useful CLI tool `aoc` which can be installed with the
`make` command. It will be installed in the default location for Go binaries
which is at `$GOPATH/bin`. Following `make` targets are provided:

* `make` (default): build and install the `aoc` binary at the default path
* `make clean`: remove the build artifacts such as the `aoc` binary
* `make tidy`: run `go mod tidy`

### Usage

```
Usage: aoc [-y <year>] [-d <day>] [-t] [-cpuprofile] [-memprofile]

Options:
  -cpuprofile
        write a CPU profile
  -d int
        run solution for given day (default 25)
  -memprofile
        write a memory profile
  -t    run the test input instead
  -y int
        run solution for given year (default 2021)
```

**Note:** The default value for `-d` (day) and `-y` (year) flags are calculated
at runtime as per the current date.

## Packages

[![Go Reference](https://pkg.go.dev/badge/github.com/dhruvmanila/advent-of-code/go.svg)](https://pkg.go.dev/github.com/dhruvmanila/advent-of-code/go)

There are various utility functions provided in the `util` package. Additional
packages which contains implementation for various data structures and
algorithms are also provided in the `pkg/` subdirectory. See the
[Documentation](https://pkg.go.dev/github.com/dhruvmanila/advent-of-code/go) for more details.
