package year2021

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

type alu struct {
	instructions []string
	vars         map[string]int
}

func newAlu(instructions []string) *alu {
	return &alu{
		instructions: instructions,
		vars: map[string]int{
			"w": 0,
			"x": 0,
			"y": 0,
			"z": 0,
		},
	}
}

func (a *alu) run(input int) {
	digits := util.Digits(input)
	for _, instruction := range a.instructions {
		fields := strings.Fields(instruction)
		opcode, result := fields[0], fields[1]

		var num int
		if len(fields) == 3 {
			i, err := strconv.Atoi(fields[2])
			if err == nil {
				num = i
			} else {
				num = a.vars[fields[2]]
			}
		}

		switch opcode {
		case "inp":
			a.vars[result] = <-digits
		case "add":
			a.vars[result] += num
		case "mul":
			a.vars[result] *= num
		case "div":
			a.vars[result] /= num
		case "mod":
			a.vars[result] %= num
		case "eql":
			if a.vars[result] == num {
				a.vars[result] = 1
			} else {
				a.vars[result] = 0
			}
		default:
			panic("invalid opcode: " + opcode)
		}
	}
}

// reset will reset all the variables back to 0.
func (a *alu) reset() {
	for v := range a.vars {
		a.vars[v] = 0
	}
}

// z is used as a stack storing a bunch of small numbers at once by treating
// it as a big base-26 number. So, (z * 26) means z.push() and (z % 26) means
// z.pop().
//
// Digit 1
//
//   w₁ <-
//   z = 26z + w₁ + 2
//
// Digit 2
//
//   w₂ <-
//   z = 26z + w₂ + 16
//
// Digit 3
//
//   w₃ <-
//   z = 26z + w₃ + 9
//
// Digit 4
//
//   w₄ <-
//   z = 26z + w₄
//
// Digit 5
//
//   w₅ <-
//   z = z/26 => (z % 26) - 8 == w₅
//            => w₄ - 8 == w₅
//
// Digit 6
//
//   w₆ <-
//   z = 26z + w₆ + 12
//
// Digit 7
//
//   w₇ <-
//   z = z/26 => (z % 26) - 16 == w₇
//            => w₆ + 12 - 16 == w₇
//            => w₆ - 4 == w₇
//
// Digit 8
//
//   w₈ <-
//   z = z/26 => (z % 26) - 4 == w₈
//            => w₃ + 9 - 4 == w₈
//            => w₃ + 5 == w₈
//
// Digit 9
//
//   w₉ <-
//   z = 26z + w₉ + 3
//
// Digit 10
//
//   w₁₀ <-
//   z = z/26 => (z % 26) - 3 == w₁₀
//            => w₉ + 3 - 3 == w₁₀
//            => w₉ == w₁₀
//
// Digit 11
//
//   w₁₁ <-
//   z = 26z + w₁₁ + 9
//
// Digit 12
//
//   w₁₂ <-
//   z = z/26 => (z % 26) - 7 == w₁₂
//            => w₁₁ + 9 - 7 == w₁₂
//            => w₁₁ + 2 == w₁₂
//
// Digit 13
//
//   w₁₃ <-
//   z = z/26 => (z % 26) - 15 == w₁₃
//            => w₂ + 16 - 15 == w₁₃
//            => w₂ + 1 == w₁₃
//
// Digit 14
//
//   w₁₄ <-
//   z = z/26 => (z % 26) - 7 == w₁₄
//            => w₁ + 2 - 7 == w₁₄
//            => w₁ - 5 == w₁₄
//
// Final conditions
//
//   w₄  - 8 == w₅
//   w₆  - 4 == w₇
//   w₃  + 5 == w₈
//   w₉      == w₁₀
//   w₁₁ + 2 == w₁₂
//   w₂  + 1 == w₁₃
//   w₁  - 5 == w₁₄
func Sol24(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	alu := newAlu(lines)
	alu.run(98491959997994)
	fmt.Printf("%+v\n", alu.vars)

	alu.reset()
	alu.run(61191516111321)
	fmt.Printf("%+v\n", alu.vars)

	fmt.Printf("24.1: %d\n24.2: %d\n", 98491959997994, 61191516111321)
	return nil
}
