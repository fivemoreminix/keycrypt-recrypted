package main

import (
	"os"
	"testing"
)

func TestDecrypt(t *testing.T) {
	expected := "  hello,  world! "
	words := []string{"hell", "hella", "worldd", "world", "yo", "hello"}
	table := MakeHashTable(words)
	if got := DecryptString("  63999,  29493! ", table); got != expected {
		t.Errorf("Expected %#v, got %#v", expected, got)
	}
}

func BenchmarkDecrypt(b *testing.B) {
	encryptedStr := EncryptString("Truthfully, I don't know whether Perry the Platypus is male or female, but I know it is handsome.")
	words, err := LoadWordList("words.txt")
	if err != nil {
		b.Errorf("Failed to load word list: %v", err)
		return
	}
	table := MakeHashTable(words)
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		DecryptString(encryptedStr, table)
	}
}

func BenchmarkLoadWordList(b *testing.B) {
	f, err := os.Open("words.txt")
	if err != nil { // Ensure the word list can be reached
		b.Errorf("Could not open words list: %v", err)
	}
	f.Close()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		LoadWordList("words.txt")
	}
}
