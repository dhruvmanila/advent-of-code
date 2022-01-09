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
		1:  year2020.Sol01,
		2:  year2020.Sol02,
		3:  year2020.Sol03,
		4:  year2020.Sol04,
		5:  year2020.Sol05,
		6:  year2020.Sol06,
		7:  year2020.Sol07,
		8:  year2020.Sol08,
		9:  year2020.Sol09,
		10: year2020.Sol10,
		11: year2020.Sol11,
		12: year2020.Sol12,
		14: year2020.Sol14,
	},
	2021: {
		1:  year2021.Sol01,
		2:  year2021.Sol02,
		3:  year2021.Sol03,
		4:  year2021.Sol04,
		5:  year2021.Sol05,
		6:  year2021.Sol06,
		7:  year2021.Sol07,
		8:  year2021.Sol08,
		9:  year2021.Sol09,
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
	useTestInput bool
	aocYear      int
	aocDay       int
	cpuprofile   bool
	memprofile   bool
)

func init() {
	year, month, day := time.Now().Date()
	switch {
	case month != time.December:
		year--
		fallthrough
	case day >= 25:
		day = 25
	}

	flag.BoolVar(&useTestInput, "t", false, "run the test input instead")
	flag.IntVar(&aocYear, "y", year, "run solution for given year")
	flag.IntVar(&aocDay, "d", day, "run solution for given day")
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

	if useTestInput {
		input = fmt.Sprintf("./year%d/input/test/%02d.txt", aocYear, aocDay)
	} else {
		input = fmt.Sprintf("./year%d/input/%02d.txt", aocYear, aocDay)
	}

	if cpuprofile {
		f, err := os.Create("cpu.prof")
		if err != nil {
			log.Fatal(err)
		}
		pprof.StartCPUProfile(f)
		defer pprof.StopCPUProfile()
	}

	if yearSolutions, exist := solutions[aocYear]; exist {
		if solution, exist := yearSolutions[aocDay]; exist {
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
		log.Fatal(fmt.Errorf("year %d: day %d: %w", aocYear, aocDay, err))
	}
}
