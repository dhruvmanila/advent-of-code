package year2021

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

type alu struct {
	instructions []string
	w, x, y, z   int
}

func newAlu(instructions []string) *alu {
	return &alu{instructions: instructions}
}

func (a *alu) get(varname string) int {
	switch varname {
	case "w":
		return a.w
	case "x":
		return a.x
	case "y":
		return a.y
	case "z":
		return a.z
	}
	panic("unknown variable: " + varname)
}

func (a *alu) set(varname string, value int) {
	switch varname {
	case "w":
		a.w = value
	case "x":
		a.x = value
	case "y":
		a.y = value
	case "z":
		a.z = value
	default:
		panic("unknown variable: " + varname)
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
				num = a.get(fields[2])
			}
		}

		switch opcode {
		case "inp":
			a.set(result, <-digits)
		case "add":
			a.set(result, a.get(result)+num)
		case "mul":
			a.set(result, a.get(result)*num)
		case "div":
			a.set(result, a.get(result)/num)
		case "mod":
			a.set(result, a.get(result)%num)
		case "eql":
			if a.get(result) == num {
				a.set(result, 1)
			} else {
				a.set(result, 0)
			}
		default:
			panic("invalid opcode: " + opcode)
		}
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
	alu.run(61191516111321)

	fmt.Printf("24.1: %d\n24.2: %d\n", 98491959997994, 61191516111321)
	return nil
}
