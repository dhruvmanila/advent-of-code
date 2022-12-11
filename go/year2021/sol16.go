package year2021

import (
	"encoding/hex"
	"fmt"
	"math"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

type expression interface {
	versionSum() int
	evaluate() int
}

// packet contains common information for multiple packet types.
type packet struct {
	// version and typeId is the version and type ID of a literal packet.
	version int
	typeId  int
}

// literalPacket contains information regarding a single literal packet.
type literalPacket struct {
	packet

	// value is the numeric value encoded in the literal packet.
	value int
}

func (p *literalPacket) versionSum() int {
	return p.version
}

func (p *literalPacket) evaluate() int {
	return p.value
}

// operatorPacket contains information regarding a single operator packet.
type operatorPacket struct {
	packet

	// subPackets is a list of packets contained within this packet. It does
	// not contain any sub packets of the containing packet.
	subPackets []expression // no union types yet :(
}

func (p *operatorPacket) versionSum() int {
	v := p.version
	for _, p := range p.subPackets {
		v += p.versionSum()
	}
	return v
}

func (p *operatorPacket) evaluate() int {
	switch p.typeId {
	case 0:
		sum := 0
		for _, sp := range p.subPackets {
			sum += sp.evaluate()
		}
		return sum
	case 1:
		product := 1
		for _, sp := range p.subPackets {
			product *= sp.evaluate()
		}
		return product
	case 2:
		minimum := math.MaxInt
		for _, sp := range p.subPackets {
			minimum = util.Min(minimum, sp.evaluate())
		}
		return minimum
	case 3:
		maximum := math.MinInt
		for _, sp := range p.subPackets {
			maximum = util.Max(maximum, sp.evaluate())
		}
		return maximum
	case 5:
		if p.subPackets[0].evaluate() > p.subPackets[1].evaluate() {
			return 1
		}
	case 6:
		if p.subPackets[0].evaluate() < p.subPackets[1].evaluate() {
			return 1
		}
	case 7:
		if p.subPackets[0].evaluate() == p.subPackets[1].evaluate() {
			return 1
		}
	}
	return 0
}

type parser struct {
	ptr int
	bin string
}

func newParser(bin string) *parser {
	return &parser{
		ptr: 0,
		bin: bin,
	}
}

func (p *parser) parse() expression {
	version, typeId := p.parseHeader()
	packet := packet{
		version: version,
		typeId:  typeId,
	}

	if typeId == 4 { // literal packet
		value := p.parseLiteralPacket()
		return &literalPacket{
			packet: packet,
			value:  value,
		}
	}

	var subPackets []expression
	lengthTypeId := p.bin[p.ptr]
	p.ptr++
	if lengthTypeId == '0' {
		subPackets = p.parseSubPacketsByLength()
	} else {
		subPackets = p.parseSubPacketsByCount()
	}
	return &operatorPacket{
		packet:     packet,
		subPackets: subPackets,
	}
}

func (p *parser) parseHeader() (int, int) {
	version := util.MustBtoi(p.bin[p.ptr : p.ptr+3])
	typeId := util.MustBtoi(p.bin[p.ptr+3 : p.ptr+6])
	p.ptr += 6
	return version, typeId
}

func (p *parser) parseLiteralPacket() int {
	var literal string
	for {
		prefix := p.bin[p.ptr]
		literal += p.bin[p.ptr+1 : p.ptr+5]
		p.ptr += 5
		if prefix == '0' {
			break
		}
	}
	return util.MustBtoi(literal)
}

func (p *parser) parseSubPacketsByLength() []expression {
	length := util.MustBtoi(p.bin[p.ptr : p.ptr+15])
	p.ptr += 15
	start := p.ptr
	var subPackets []expression
	for p.ptr-start < length {
		subPackets = append(subPackets, p.parse())
	}
	return subPackets
}

func (p *parser) parseSubPacketsByCount() []expression {
	count := util.MustBtoi(p.bin[p.ptr : p.ptr+11])
	p.ptr += 11
	var subPackets []expression
	for i := 0; i < count; i++ {
		subPackets = append(subPackets, p.parse())
	}
	return subPackets
}

func (p *parser) reset() {
	p.ptr = 0
}

// hexToBinary is used to convert the given hex string into an equivalent
// binary string.
func hexToBinary(h string) (string, error) {
	bs, err := hex.DecodeString(h)
	if err != nil {
		return "", err
	}
	var s string
	for _, b := range bs {
		s += fmt.Sprintf("%08b", b)
	}
	return s, nil
}

func Sol16(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}

	h, err := hexToBinary(lines[0])
	if err != nil {
		return "", err
	}

	p := newParser(h)
	packet := p.parse()

	return fmt.Sprintf("16.1: %d\n16.2: %d\n", packet.versionSum(), packet.evaluate()), nil
}
