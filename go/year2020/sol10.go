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

	diffCounter := counter.New()
	diffCounter.Add(ratings[0] - effectiveRating)
	for i := 0; i < len(ratings)-1; i++ {
		diffCounter.Add(ratings[i+1] - ratings[i])
	}
	diffCounter.Add(3)

	fmt.Printf(
		"10.1: %d\n10.2: %d\n",
		diffCounter.Get(1)*diffCounter.Get(3),
		arrangementCount(effectiveRating, ratings, make(map[int]int)),
	)
	return nil
}
