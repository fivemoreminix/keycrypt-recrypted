package main

import (
	"regexp"
	"strings"
)

var wordRegexp = regexp.MustCompile("[0-9]+")

// decryptWord assumes word has nothing but digits representing an encrypted
// word.
func decryptWord(word string, words map[string]string) string {
	if w := words[word]; w != "" {
		return w
	}
	return strings.Repeat("?", len(word))
}

// DecryptString takes a string of encrypted words and symbols, and decrypts
// the message, keeping the placement of symbols.
func DecryptString(str string, words map[string]string) string {
	var sb strings.Builder
	sb.Grow(len(str))
	wordBounds := wordRegexp.FindAllStringIndex(str, -1)
	var prevWordEnd int

	for _, wordBound := range wordBounds {
		sb.WriteString(str[prevWordEnd:wordBound[0]])                      // Write contents before the current word, and the last word
		sb.WriteString(decryptWord(str[wordBound[0]:wordBound[1]], words)) // Write the decrypted word
		prevWordEnd = wordBound[1]
	}
	sb.WriteString(str[prevWordEnd:]) // Write any remaining whitespace or symbols after last word
	return sb.String()
}
