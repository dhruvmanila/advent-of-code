package year2016

import (
	"crypto/md5"
	"encoding/hex"
	"fmt"
)

const input = "cxdnnyjw"

func Sol05(_ string) error {
	password1 := make([]byte, 0, 8)
	passwordLetters := make(map[int]byte, 8)
	for i := 0; len(password1) != 8 || len(passwordLetters) != 8; i++ {
		sum := md5.Sum([]byte(fmt.Sprintf("%s%d", input, i)))
		if sum[0] == 0 && sum[1] == 0 && sum[2]&0xF0 == 0 {
			hexStr := hex.EncodeToString(sum[:])
			if len(password1) != 8 {
				password1 = append(password1, hexStr[5])
			}
			if len(passwordLetters) != 8 {
				position := int(hexStr[5] - '0')
				switch position {
				case 0, 1, 2, 3, 4, 5, 6, 7:
					if _, ok := passwordLetters[position]; !ok {
						passwordLetters[position] = hexStr[6]
					}
				}
			}
		}
	}

	password2 := make([]byte, 8)
	for position, letter := range passwordLetters {
		password2[position] = letter
	}

	fmt.Printf("5.1: %s\n5.2: %s\n", string(password1), string(password2))
	return nil
}
