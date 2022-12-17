package main

import (
	"errors"
	"flag"
	"fmt"
	"io/fs"
	"log"
	"os"
	"runtime/pprof"
	"strings"
	"text/template"
	"time"

	"github.com/dhruvmanila/advent-of-code/go/year2016"
	"github.com/dhruvmanila/advent-of-code/go/year2020"
	"github.com/dhruvmanila/advent-of-code/go/year2021"
	"github.com/dhruvmanila/advent-of-code/go/year2022"
)

var errUnsolved = errors.New("unsolved")

type solutionFunc func(string) (string, error)

// solutions is a map from year to day to the solution function.
var solutions = map[int]map[int]solutionFunc{
	2016: {
		1:  year2016.Sol01,
		2:  year2016.Sol02,
		3:  year2016.Sol03,
		4:  year2016.Sol04,
		5:  year2016.Sol05,
		6:  year2016.Sol06,
		7:  year2016.Sol07,
		8:  year2016.Sol08,
		9:  year2016.Sol09,
		10: year2016.Sol10,
	},
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
		13: year2020.Sol13,
		14: year2020.Sol14,
		15: year2020.Sol15,
		16: year2020.Sol16,
		17: year2020.Sol17,
		18: year2020.Sol18,
		19: year2020.Sol19,
		20: year2020.Sol20,
		21: year2020.Sol21,
		22: year2020.Sol22,
		23: year2020.Sol23,
		24: year2020.Sol24,
		25: year2020.Sol25,
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
	2022: {
		1:  year2022.Sol01,
		2:  year2022.Sol02,
		3:  year2022.Sol03,
		4:  year2022.Sol04,
		5:  year2022.Sol05,
		6:  year2022.Sol06,
		7:  year2022.Sol07,
		8:  year2022.Sol08,
		9:  year2022.Sol09,
		10: year2022.Sol10,
		11: year2022.Sol11,
		12: year2022.Sol12,
		13: year2022.Sol13,
		14: year2022.Sol14,
		15: year2022.Sol15,
	},
}

// Command line options.
var (
	useTestInput bool
	aocYear      int
	aocDay       int
	cpuprofile   bool
	memprofile   bool
	runs         int
	timeSolution bool
)

func init() {
	year, month, day := time.Now().Date()
	switch {
	case month != time.December:
		year--
		fallthrough
	case day > 25:
		day = 25
	}

	flag.BoolVar(&useTestInput, "t", false, "run the test input instead")
	flag.IntVar(&aocYear, "y", year, "run solution for given year")
	flag.IntVar(&aocDay, "d", day, "run solution for given day")
	flag.BoolVar(&cpuprofile, "cpuprofile", false, "write a CPU profile")
	flag.BoolVar(&memprofile, "memprofile", false, "write a memory profile")
	flag.IntVar(&runs, "runs", 100, "run solution n times for profiling")
	flag.BoolVar(&timeSolution, "time", false, "time the solution")
}

func usage() {
	fmt.Fprintf(os.Stderr, `Usage: %s [OPTIONS]

Options:
`, os.Args[0])
	flag.PrintDefaults()
}

func main() {
	// Call realMain instead of doing the work here so we can use `defer`
	// statements within the function and have them work properly.
	// (defers aren't called with os.Exit)
	os.Exit(realMain())
}

func realMain() int {
	log.SetPrefix("aoc: ")
	log.SetFlags(0)

	flag.Usage = usage
	flag.Parse()

	var input string

	if useTestInput {
		input = fmt.Sprintf("./year%d/input/test/%02d.txt", aocYear, aocDay)
	} else {
		input = fmt.Sprintf("./year%d/input/%02d.txt", aocYear, aocDay)
	}

	if cpuprofile {
		f, err := os.Create("cpu.prof")
		if err != nil {
			log.Print(err)
			return 1
		}
		if err := pprof.StartCPUProfile(f); err != nil {
			log.Print(err)
			return 1
		}
	}

	var s string
	var solutionErr error

	if yearSolutions, exist := solutions[aocYear]; exist {
		if solution, exist := yearSolutions[aocDay]; exist {
			// If profiling is turned on, show the time it took to profile. If
			// it's off, then the solution should only run one time.
			if cpuprofile || memprofile {
				timeSolution = true
			} else {
				runs = 1
			}

			var start time.Time
			if timeSolution {
				start = time.Now()
			}

			for i := 0; i < runs; i++ {
				s, solutionErr = solution(input)
				// Stop re-running the solution if there's an error.
				if solutionErr != nil && cpuprofile {
					log.Println("error in solution: profiling stopped")
					break
				}
			}

			// This is safe to call without starting the profiler. It is called
			// here so as to only profile the solution function.
			pprof.StopCPUProfile()

			if timeSolution {
				s = fmt.Sprintf("%s> %s\n", s, time.Since(start))
			}
		} else {
			solutionErr = errUnsolved
		}
	} else {
		solutionErr = errUnsolved
	}

	if errors.Is(solutionErr, errUnsolved) {
		var response string

		fmt.Printf("./year%d/sol%02d.go does not exist. Generate? [y/n]: ", aocYear, aocDay)
		_, err := fmt.Scan(&response)
		if err != nil {
			log.Print(err)
			return 1
		}

		switch strings.ToLower(strings.TrimSpace(response)) {
		case "y", "yes":
			err := createSolution(aocYear, aocDay)
			if err != nil {
				log.Print(err)
				return 1
			}
			return 0
		default:
			return 1
		}
	}

	if memprofile {
		f, err := os.Create("mem.prof")
		if err != nil {
			log.Print(err)
			return 1
		}
		defer f.Close()

		if err := pprof.WriteHeapProfile(f); err != nil {
			log.Print(err)
			return 1
		}
	}

	if solutionErr != nil {
		log.Print(fmt.Errorf("year %d: day %d: %w", aocYear, aocDay, solutionErr))
		return 1
	}

	fmt.Print(s)
	return 0
}

func createSolution(year, day int) error {
	yearDir := fmt.Sprintf("./year%d", year)
	if _, err := os.Stat(yearDir); errors.Is(err, fs.ErrNotExist) {
		os.MkdirAll(fmt.Sprintf("./year%d/input/test", year), 0o755)
	}

	// Create the input files.
	for _, filepath := range []string{
		fmt.Sprintf("./year%d/input/test/%02d.txt", year, day),
		fmt.Sprintf("./year%d/input/%02d.txt", year, day),
	} {
		f, err := os.Create(filepath)
		if err != nil {
			return err
		}
		f.Close()
	}

	t, err := template.ParseFiles("templates/solution")
	if err != nil {
		return err
	}

	f, err := os.OpenFile(fmt.Sprintf("./year%d/sol%02d.go", year, day), os.O_CREATE|os.O_WRONLY, 0o644)
	if err != nil {
		return err
	}
	defer f.Close()

	if err := t.Execute(f, map[string]int{"Year": year, "Day": day}); err != nil {
		return err
	}

	return nil
}
