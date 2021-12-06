package main

import (
	"errors"
	"fmt"
	"log"
	"os"
	"strconv"

	"github.com/dhruvmanila/advent-of-code/go/year2020"
	"github.com/dhruvmanila/advent-of-code/go/year2021"
)

// ErrUnsolved is returned if the problem for the specific year and day is
// not solved yet.
var ErrUnsolved = errors.New("unsolved")

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

	input := fmt.Sprintf("./year%d/input/%02d.txt", year, day)
	switch year {
	case 2020:
		switch day {
		case 1:
			err = year2020.Sol1(input)
		case 2:
			err = year2020.Sol2(input)
		default:
			err = ErrUnsolved
		}
	case 2021:
		switch day {
		case 1:
			err = year2021.Sol1(input)
		case 2:
			err = year2021.Sol2(input)
		case 3:
			err = year2021.Sol3(input)
		case 4:
			err = year2021.Sol4(input)
		default:
			err = ErrUnsolved
		}
	default:
		err = ErrUnsolved
	}

	if err != nil {
		log.Fatal(fmt.Errorf("year%d: Sol%d: %w", year, day, err))
	}
}
