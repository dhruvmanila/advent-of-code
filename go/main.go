package main

import (
	"fmt"
	"log"
	"os"
	"strconv"

	"github.com/dhruvmanila/advent-of-code/go/year2021"
)

func main() {
	log.SetPrefix("aoc: ")
	log.SetFlags(0)

	if len(os.Args) == 1 {
		log.Fatal("puzzle number missing")
	}

	puzzle, err := strconv.Atoi(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}

	switch puzzle {
	case 1:
		err = year2021.Sol1("./year2021/input/01.txt")
	case 2:
		err = year2021.Sol2("./year2021/input/02.txt")
	default:
		log.Fatalf("puzzle number %d: unsolved", puzzle)
	}

	if err != nil {
		log.Fatal(fmt.Errorf("sol%d: %w", puzzle, err))
	}
}
