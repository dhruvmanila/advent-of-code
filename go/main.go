package main

import (
	"fmt"
	"log"
	"os"
	"strconv"

	"github.com/dhruvmanila/advent-of-code/go/year2020"
	"github.com/dhruvmanila/advent-of-code/go/year2021"
)

// UnsolvedError is returned if the problem for the specific year and/or day is
// not solved yet.
type UnsolvedError struct {
	year int
	day  int
}

func (e *UnsolvedError) Error() string {
	message := ""
	if e.year != 0 {
		message += fmt.Sprintf("year%d: ", e.year)
	}
	if e.day != 0 {
		message += fmt.Sprintf("Sol%d: ", e.day)
	}
	return message + "unsolved"
}

type solutionFunc func(string) error

// solutions is a map from year to day to the solution function.
var solutions = map[int]map[int]solutionFunc{
	2020: {
		1: year2020.Sol1,
		2: year2020.Sol2,
	},
	2021: {
		1: year2021.Sol1,
		2: year2021.Sol2,
		3: year2021.Sol3,
		4: year2021.Sol4,
		5: year2021.Sol5,
		6: year2021.Sol6,
		7: year2021.Sol7,
		8: year2021.Sol8,
	},
}

func main() {
	log.SetPrefix("aoc: ")
	log.SetFlags(0)

	if len(os.Args) != 3 {
		log.Fatalf("Usage: %s <year> <day>", os.Args[0])
	}

	year, err := strconv.Atoi(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}

	day, err := strconv.Atoi(os.Args[2])
	if err != nil {
		log.Fatal(err)
	}

	var input string
	if os.Getenv("TEST") == "" {
		input = fmt.Sprintf("./year%d/input/%02d.txt", year, day)
	} else {
		input = fmt.Sprintf("./year%d/input/test/%02d.txt", year, day)
	}

	if yearSolutions, exist := solutions[year]; exist {
		if solution, exist := yearSolutions[day]; exist {
			err = solution(input)
		} else {
			err = &UnsolvedError{year: year, day: day}
		}
	} else {
		err = &UnsolvedError{year: year}
	}

	if err != nil {
		log.Fatal(err)
	}
}
