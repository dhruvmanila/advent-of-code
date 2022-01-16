package year2020

import (
	"fmt"
	"sort"

	"github.com/dhruvmanila/advent-of-code/go/pkg/counter"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

// arrangementCount is used to count the number of possible arragements of the
// adapters that connect the charging outlet to the device.
//
// This function can be explained by a simple example:
//     previous := 0        // effective rating
//     ratings  := [1, 3]
//
// Now, to find the possible arrangements for the three ratings, we will find
// the arragements for two ratings first (previous = 1, ratings = [3]). There
// is only one way for that to happen (0 -> 3).
//
// Then, we will consider the previous rating (0) and see if the middle rating
// is optional by checking if the difference conforms the rules. 3 - 0 = 3
// which is <= 3. So, we should include all the arrangements without the middle
// number as well. (previous = 0, ratings = [3])
//
//     0 -> 3
//     0 -> 1 -> 3
func arrangementCount(previous int, ratings []int, memo map[int]int) int {
	if len(ratings) == 1 {
		return 1
	}
	if count, ok := memo[previous]; ok {
		return count
	}
	count := arrangementCount(ratings[0], ratings[1:], memo)
	if ratings[1]-previous <= 3 {
		count += arrangementCount(previous, ratings[1:], memo)
	}
	memo[previous] = count
	return count
}

func Sol10(input string) error {
	ratings, err := util.ReadLinesAsInt(input)
	if err != nil {
		return err
	}

	effectiveRating := 0
	sort.Ints(ratings)

	// dc is a difference counter.
	dc := counter.New[int]()
	dc.Increment(ratings[0] - effectiveRating)
	for i := 0; i < len(ratings)-1; i++ {
		dc.Increment(ratings[i+1] - ratings[i])
	}
	dc.Increment(3)

	fmt.Printf(
		"10.1: %d\n10.2: %d\n",
		dc.Get(1)*dc.Get(3),
		arrangementCount(effectiveRating, ratings, make(map[int]int)),
	)
	return nil
}
