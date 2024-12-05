package year2016

import (
	"fmt"
	"regexp"
	"sort"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/pkg/counter"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

var roomRegex = regexp.MustCompile(`^([a-z-]+)-(\d+)\[([a-z]+)\]$`)

type pair struct {
	key   byte
	value int
}

type roomInfo struct {
	name     string
	sectorId int
	checksum string
}

func newRoomInfoFromLine(line string) (*roomInfo, error) {
	matches := roomRegex.FindStringSubmatch(line)
	if matches == nil || len(matches) != 4 {
		return nil, fmt.Errorf("%q: invalid match: %v", line, matches)
	}
	return &roomInfo{
		name:     matches[1],
		sectorId: util.MustAtoi(matches[2]),
		checksum: matches[3],
	}, nil
}

func (r *roomInfo) isReal() bool {
	letterCount := counter.NewFromSlice([]byte(r.name))
	letterCount.Delete('-')

	pairs := make([]pair, 0, letterCount.Len())
	letterCount.ForEach(func(item byte, count int) {
		pairs = append(pairs, pair{item, count})
	})

	sort.Slice(pairs, func(i, j int) bool {
		if pairs[i].value == pairs[j].value {
			return pairs[i].key < pairs[j].key
		}
		// Sort in descending order
		return pairs[i].value > pairs[j].value
	})

	checksum := make([]byte, 5)
	for i, pair := range pairs[:5] {
		checksum[i] = pair.key
	}
	return string(checksum) == r.checksum
}

func (r *roomInfo) decrypt() string {
	return strings.Map(func(letter rune) rune {
		switch letter {
		case '-':
			return ' '
		default:
			shifted := int(letter) + r.sectorId%26
			if shifted > 'z' {
				return rune(shifted - 26)
			}
			return rune(shifted)
		}
	}, r.name)
}

func Sol04(input string) (string, error) {
	lines := util.ReadLines(input)

	rooms := make([]*roomInfo, 0, len(lines))
	for _, line := range lines {
		room, err := newRoomInfoFromLine(line)
		if err != nil {
			return "", err
		}
		rooms = append(rooms, room)
	}

	sum := 0
	var id int
	for _, room := range rooms {
		if room.isReal() {
			if room.decrypt() == "northpole object storage" {
				id = room.sectorId
			}
			sum += room.sectorId
		}
	}

	return fmt.Sprintf("4.1: %d\n4.2: %d\n", sum, id), nil
}
