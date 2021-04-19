package main

import (
	"strings"
)

var EncodeLookup = map[rune]int{
	'q': 1, 'a': 1, 'z': 1,
	'w': 2, 's': 2, 'x': 2,
	'e': 3, 'd': 3, 'c': 3,
	'r': 4, 'f': 4, 'v': 4,
	't': 5, 'g': 5, 'b': 5,
	'y': 6, 'h': 6, 'n': 6,
	'u': 7, 'j': 7, 'm': 7,
	'i': 8, 'k': 8,
	'o': 9, 'l': 9,
	'p': 0,
}

func EncryptString(str string) string {
	var sb strings.Builder
	sb.Grow(len(str))
	for _, r := range str {
		if (r >= 'A' && r <= 'Z') || (r >= 'a' && r <= 'z') {
			if r < 'a' {
				r += 'a' - 'A'
			}
			sb.WriteByte((byte)(EncodeLookup[r] + '0'))
		} else {
			sb.WriteRune(r)
		}
	}
	return sb.String()
}
