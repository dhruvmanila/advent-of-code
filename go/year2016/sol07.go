package year2016

import (
	"fmt"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

type ipAddress struct {
	// supernet are sequences which lies outside the square bracket of an
	// IP Address.
	supernet []string

	// hypernet are sequences which lies inside the square bracket of an
	// IP Address.
	hypernet []string
}

func newIpAddressFromLine(line string) *ipAddress {
	fields := strings.FieldsFunc(line, func(r rune) bool {
		return r == ']' || r == '['
	})

	supernet := make([]string, 0, len(fields))
	hypernet := make([]string, 0, len(fields))

	for i, field := range fields {
		switch {
		case i%2 == 0:
			supernet = append(supernet, field)
		default:
			hypernet = append(hypernet, field)
		}
	}

	return &ipAddress{
		supernet: supernet,
		hypernet: hypernet,
	}
}

func (addr *ipAddress) supportsTLS() bool {
	supported := false
	for _, seq := range addr.supernet {
		for i := 0; i <= len(seq)-4; i++ {
			if hasABBA(seq[i : i+4]) {
				supported = true
				break
			}
		}
	}
	for _, seq := range addr.hypernet {
		for i := 0; i <= len(seq)-4; i++ {
			if hasABBA(seq[i : i+4]) {
				supported = false
				break
			}
		}
	}
	return supported
}

func (addr *ipAddress) supportsSSL() bool {
	candidates := set.New[string]()
	for _, seq := range addr.supernet {
		for i := 0; i <= len(seq)-3; i++ {
			if hasABA(seq[i : i+3]) {
				candidates.Add(string([]byte{seq[i+1], seq[i], seq[i+1]}))
			}
		}
	}
	if candidates.Len() == 0 {
		return false
	}

	supported := false
	for _, seq := range addr.hypernet {
		for i := 0; i <= len(seq)-3; i++ {
			if s := seq[i : i+3]; hasABA(s) && candidates.Contains(s) {
				supported = true
				break
			}
		}
	}
	return supported
}

func hasABBA(s string) bool {
	return s[0] == s[3] && s[1] == s[2] && s[0] != s[1] && s[2] != s[3]
}

func hasABA(s string) bool {
	return s[0] == s[2] && s[0] != s[1]
}

func Sol07(input string) (string, error) {
	lines := util.ReadLines(input)

	ipAddresses := make([]*ipAddress, 0, len(lines))
	for _, line := range lines {
		ipAddresses = append(ipAddresses, newIpAddressFromLine(line))
	}

	tlsCount := 0
	sslCount := 0
	for _, ipaddr := range ipAddresses {
		if ipaddr.supportsTLS() {
			tlsCount++
		}
		if ipaddr.supportsSSL() {
			sslCount++
		}
	}

	return fmt.Sprintf("7.1: %d\n7.2: %d\n", tlsCount, sslCount), nil
}
