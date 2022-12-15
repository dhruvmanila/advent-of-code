package year2022

import (
	"fmt"
	"sort"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/pkg/iterator"
	"github.com/dhruvmanila/advent-of-code/go/pkg/stack"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

const (
	dividerPacketTwo = "[[2]]"
	dividerPacketSix = "[[6]]"
)

// packetTokenizer is used to generate the tokens for the packet.
// Valid tokens are: "[", "]", "," and any digits from 0-9.
type packetTokenizer struct {
	packet *iterator.Iterator[string]
	stack  *stack.Stack[string]
}

func newPacketTokenizer(packet string) *packetTokenizer {
	return &packetTokenizer{
		packet: iterator.New(strings.Split(packet, "")),
		stack:  stack.New[string](),
	}
}

// next returns the next token for the packet. It panics when an invalid
// token is found.
func (t *packetTokenizer) next() string {
	if val, ok := t.stack.Pop(); ok {
		return val
	}
	for t.packet.Next() {
		switch token := t.packet.Value(); token {
		case ",":
			continue
		case "]", "[":
			return token
		case "0", "1", "2", "3", "4", "5", "6", "7", "8", "9":
		NextLoop:
			// Collect the next digits to form the entire number.
			for t.packet.Next() {
				switch nextToken := t.packet.Value(); nextToken {
				case ",":
					break NextLoop
				case "]":
					// Now that the number is complete and we've found a
					// token, push it on to the stack for it to be returned
					// on the next call.
					t.stack.Push(nextToken)
					break NextLoop
				case "0", "1", "2", "3", "4", "5", "6", "7", "8", "9":
					token += nextToken
				}
			}
			return token
		default:
			panic(fmt.Sprintf("%q: illegal token", token))
		}
	}
	return ""
}

// pushback pushes the given number on to the stack. This basically converts
// the number packet to a list packet.
func (t *packetTokenizer) pushback(number string) {
	// To convert it to a list packet, we need the closing bracket to occur
	// after the number. We've already consumed the open bracket.
	t.stack.Push("]")
	t.stack.Push(number)
}

// Less returns true if the lhs packet is less than the rhs packet as per
// the rules stated in the problem statement.
func Less(lhs string, rhs string) bool {
	p1, p2 := newPacketTokenizer(lhs), newPacketTokenizer(rhs)

	for t1, t2 := p1.next(), p2.next(); t1 != "" && t2 != ""; t1, t2 = p1.next(), p2.next() {
		switch {
		case t1 == "[" && t2 == "[":
			continue
		case t1 == "]" && t2 == "]":
			continue
		case t1 == "]" || t1 == "":
			return true
		case t2 == "]" || t2 == "":
			return false
		case t1 == "[":
			p2.pushback(t2)
		case t2 == "[":
			p1.pushback(t1)
		default:
			n1, n2 := util.MustAtoi(t1), util.MustAtoi(t2)
			if n1 == n2 {
				continue
			}
			return n1 < n2
		}
	}

	return false
}

func Sol13(input string) (string, error) {
	pairs, err := util.ReadSections(input)
	if err != nil {
		return "", err
	}

	// Collect all the packets to sort it later. The capacity includes the
	// divider packets to be added for the second part.
	packets := make([]string, 0, len(pairs)*2+2)

	orderedIndexSum := 0
	for idx, pair := range pairs {
		lhs, rhs := pair[0], pair[1]
		packets = append(packets, lhs, rhs)
		if Less(lhs, rhs) {
			orderedIndexSum += idx + 1 // Packet pairs are 1-indexed
		}
	}

	// Add the divider packets.
	packets = append(packets, dividerPacketTwo, dividerPacketSix)

	sort.Slice(packets, func(i, j int) bool {
		return Less(packets[i], packets[j])
	})

	decoderKey := 1
	for idx, packet := range packets {
		switch packet {
		case dividerPacketTwo, dividerPacketSix:
			decoderKey *= (idx + 1) // Packets are 1-indexed
		}
	}

	return fmt.Sprintf("13.1: %d\n13.2: %d\n", orderedIndexSum, decoderKey), nil
}
