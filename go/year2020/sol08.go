package year2020

import (
	"errors"
	"fmt"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

// errInfiniteLoop is returned when an infinite loop is detected in the program.
var errInfiniteLoop = errors.New("infinite loop detected")

// instruction contains information regarding a single instruction.
type instruction struct {
	// op is an operation to be performed.
	op string

	// arg is an argument to the operation.
	arg int
}

type program struct {
	// instructions is a list of instruction.
	instructions []*instruction

	// accumulator is a global value updated by the "acc" operation.
	accumulator int

	// ptr is the index of the currently executing instruction.
	ptr int
}

// newProgram is used to construct a new program from a given set of instructions.
func newProgram(instructions []*instruction) *program {
	return &program{
		instructions: instructions,
		accumulator:  0,
		ptr:          0,
	}
}

// newProgramFromCode is used to construct a new program from the code.
func newProgramFromCode(lines []string) *program {
	instructions := make([]*instruction, len(lines))
	for i, line := range lines {
		data := strings.Fields(line)
		instructions[i] = &instruction{
			op:  data[0],
			arg: util.MustAtoi(data[1]),
		}
	}
	return newProgram(instructions)
}

// run is used to run the program. The program starts from wherever "pts"
// points to. errInfiniteLoop is returned when an infinite loop is detected
// while running the program.
func (p *program) run() error {
	// executed is a set of instructions which got executed. This is to detect
	// an infinite loop.
	executed := set.New[int]()

	for p.ptr < len(p.instructions) {
		if executed.Contains(p.ptr) {
			return errInfiniteLoop
		}

		executed.Add(p.ptr)
		cmd := p.instructions[p.ptr]
		switch cmd.op {
		case "jmp":
			p.ptr += cmd.arg
		case "acc":
			p.accumulator += cmd.arg
			fallthrough
		case "nop":
			p.ptr++
		default:
			return errors.New("invalid operation: " + cmd.op)
		}
	}

	return nil
}

// reset is used to reset the program back to its initial state. This resets
// the global accumulator value and the pointer.
func (p *program) reset() {
	p.accumulator = 0
	p.ptr = 0
}

func Sol08(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	p := newProgramFromCode(lines)
	if err := p.run(); err != nil {
		if errors.Is(err, errInfiniteLoop) {
			fmt.Printf("8.1: %d\n", p.accumulator)
		} else {
			return err
		}
	}

	for _, instruction := range p.instructions {
		original := instruction.op
		switch instruction.op {
		case "jmp":
			instruction.op = "nop"
		case "nop":
			instruction.op = "jmp"
		default:
			continue
		}

		p.reset()
		if err := p.run(); err == nil {
			break
		}

		instruction.op = original
	}

	fmt.Printf("8.2: %d\n", p.accumulator)
	return nil
}
