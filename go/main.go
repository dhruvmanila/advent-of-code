package main

import (
	"errors"
	"flag"
	"fmt"
	"log"
	"os"
	"runtime/pprof"
	"time"

	"github.com/dhruvmanila/advent-of-code/go/year2020"
	"github.com/dhruvmanila/advent-of-code/go/year2021"
)

var errUnsolved = errors.New("unsolved")

type solutionFunc func(string) error

// solutions is a map from year to day to the solution function.
var solutions = map[int]map[int]solutionFunc{
	2020: {
		1:  year2020.Sol1,
		2:  year2020.Sol2,
		3:  year2020.Sol3,
		4:  year2020.Sol4,
		5:  year2020.Sol5,
		6:  year2020.Sol6,
		7:  year2020.Sol7,
		8:  year2020.Sol8,
		9:  year2020.Sol9,
		10: year2020.Sol10,
		11: year2020.Sol11,
	},
	2021: {
		1:  year2021.Sol1,
		2:  year2021.Sol2,
		3:  year2021.Sol3,
		4:  year2021.Sol4,
		5:  year2021.Sol5,
		6:  year2021.Sol6,
		7:  year2021.Sol7,
		8:  year2021.Sol8,
		9:  year2021.Sol9,
		10: year2021.Sol10,
		11: year2021.Sol11,
		12: year2021.Sol12,
		13: year2021.Sol13,
		14: year2021.Sol14,
		15: year2021.Sol15,
		16: year2021.Sol16,
		17: year2021.Sol17,
		18: year2021.Sol18,
		19: year2021.Sol19,
		20: year2021.Sol20,
		21: year2021.Sol21,
		22: year2021.Sol22,
		23: year2021.Sol23,
		24: year2021.Sol24,
		25: year2021.Sol25,
	},
}

// Command line options.
var (
	test       bool
	year       int
	day        int
	cpuprofile bool
	memprofile bool
)

func init() {
	now := time.Now()
	flag.BoolVar(&test, "t", false, "run the test input instead")
	flag.IntVar(&year, "y", now.Year(), "run solution for given year")
	flag.IntVar(&day, "d", now.Day(), "run solution for given day")
	flag.BoolVar(&cpuprofile, "cpuprofile", false, "write a CPU profile")
	flag.BoolVar(&memprofile, "memprofile", false, "write a memory profile")
}

func usage() {
	fmt.Fprintf(os.Stderr, `Usage: %s [-y <year>] [-d <day>] [-t] [-cpuprofile] [-memprofile]

Options:
`, os.Args[0])
	flag.PrintDefaults()
}

func main() {
	log.SetPrefix("aoc: ")
	log.SetFlags(0)

	flag.Usage = usage
	flag.Parse()

	var err error
	var input string

	if test {
		input = fmt.Sprintf("./year%d/input/test/%02d.txt", year, day)
	} else {
		input = fmt.Sprintf("./year%d/input/%02d.txt", year, day)
	}

	if cpuprofile {
		f, err := os.Create("cpu.prof")
		if err != nil {
			log.Fatal(err)
		}
		pprof.StartCPUProfile(f)
		defer pprof.StopCPUProfile()
	}

	if yearSolutions, exist := solutions[year]; exist {
		if solution, exist := yearSolutions[day]; exist {
			err = solution(input)
		} else {
			err = errUnsolved
		}
	} else {
		err = errUnsolved
	}

	if memprofile {
		f, err := os.Create("mem.prof")
		if err != nil {
			log.Fatal(err)
		}
		pprof.WriteHeapProfile(f)
		f.Close()
	}

	if err != nil {
		log.Fatal(fmt.Errorf("year %d: day %d: %w", year, day, err))
	}
}
