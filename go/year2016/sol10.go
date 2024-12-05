package year2016

import (
	"fmt"
	"strings"
	"sync"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

// These global variables are not safe for concurrent use but its fine because
// bots is only going to be updated after creating the bot and only one bot will
// write to a single output bin.
var (
	// bots is a mapping from bot id to the respective bot object.
	bots = make(map[int]*bot)

	// output is a mapping from the output bin to the respective value.
	output = make(map[int]int)
)

type bot struct {
	id int

	// inputCh is a channel for receiving the input value for the bot.
	inputCh chan int

	// lowTargetName is the target name for lower-value which can be either
	// bot or output.
	lowTargetName string
	lowTargetId   int

	// highTargetName is the target name for higher-value which can be either
	// bot or output.
	highTargetName string
	highTargetId   int
}

func (b *bot) start(wg *sync.WaitGroup) {
	wg.Add(1)
	go func() {
		// The bot proceeds only when it has two microchips.
		inputLow, inputHigh := <-b.inputCh, <-b.inputCh
		if inputLow > inputHigh {
			inputLow, inputHigh = inputHigh, inputLow
		}

		if inputLow == 17 && inputHigh == 61 {
			fmt.Printf("10.1: %d\n", b.id)
		}

		switch b.lowTargetName {
		case "bot":
			bots[b.lowTargetId].inputCh <- inputLow
		case "output":
			output[b.lowTargetId] = inputLow
		}

		switch b.highTargetName {
		case "bot":
			bots[b.highTargetId].inputCh <- inputHigh
		case "output":
			output[b.highTargetId] = inputHigh
		}

		wg.Done()
	}()
}

func executeInstructions(instructions []string) error {
	var wg sync.WaitGroup
	inputs := make([][2]int, 0)

	for _, instruction := range instructions {
		switch parts := strings.SplitN(instruction, " ", 2); parts[0] {
		case "value":
			var chip, id int
			_, err := fmt.Sscanf(parts[1], "%d goes to bot %d", &chip, &id)
			if err != nil {
				return err
			}
			inputs = append(inputs, [2]int{id, chip})
		case "bot":
			var id, lowTargetId, highTargetId int
			var lowTargetName, highTargetName string
			_, err := fmt.Sscanf(parts[1], "%d gives low to %s %d and high to %s %d",
				&id, &lowTargetName, &lowTargetId, &highTargetName, &highTargetId,
			)
			if err != nil {
				return err
			}
			bots[id] = &bot{
				id:             id,
				inputCh:        make(chan int),
				lowTargetName:  lowTargetName,
				lowTargetId:    lowTargetId,
				highTargetName: highTargetName,
				highTargetId:   highTargetId,
			}
			bots[id].start(&wg)
		default:
			return fmt.Errorf("invalid instruction: %q", instruction)
		}
	}

	for _, input := range inputs {
		bots[input[0]].inputCh <- input[1]
	}

	wg.Wait()
	return nil
}

func Sol10(input string) (string, error) {
	lines := util.ReadLines(input)

	if err := executeInstructions(lines); err != nil {
		return "", err
	}

	return fmt.Sprintf("10.2: %d\n", output[0]*output[1]*output[2]), nil
}
