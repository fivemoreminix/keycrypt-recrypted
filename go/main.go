package main

import (
	"flag"
	"fmt"
	"io/ioutil"
	"os"
	"runtime/pprof"
	"strings"
)

var (
	mode       = flag.String("mode", "decrypt", "encrypt or decrypt")
	wordlist   = flag.String("wordlist", "", "path to a file with words on separate lines to be used in decrypting")
	cpuprofile = flag.String("cpuprofile", "", "write cpu profile to file")
)

func main() {
	flag.Parse()

	if *cpuprofile != "" {
		f, err := os.Create(*cpuprofile)
		if err != nil {
			fmt.Fprintln(os.Stderr, err)
			os.Exit(1)
		}
		pprof.StartCPUProfile(f)
		defer pprof.StopCPUProfile()
	}

	var readFromStdin bool

	info, err := os.Stdin.Stat()
	if err == nil && info.Mode()&os.ModeCharDevice == 0 {
		readFromStdin = true // Using a piped input, most likely
	}

	if !readFromStdin && flag.NArg() < 1 {
		fmt.Fprintln(os.Stderr, "Expected a message to "+*mode+". Add \"your message\" to the arguments.")
		flag.Usage()
		os.Exit(1)
	}

	var message string
	if readFromStdin {
		bytes, err := ioutil.ReadAll(os.Stdin)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Failed to read input from pipe: %v", err)
			os.Exit(1)
		}
		message = string(bytes)
	} else {
		message = flag.Arg(0)
	}

	switch *mode {
	case "encrypt":
		fmt.Println(EncryptString(message))
	case "decrypt":
		words, err := LoadWordList(*wordlist)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Could not open file: %v\n", err)
			os.Exit(1)
		}
		fmt.Println(DecryptString(message, MakeHashTable(words)))
	default:
		fmt.Fprintln(os.Stderr, "Expected mode as \"encrypt\" or \"decrypt\"")
		flag.Usage()
		os.Exit(1)
	}
}

func LoadWordList(filename string) ([]string, error) {
	f, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer f.Close()
	data, err := ioutil.ReadAll(f)
	if err != nil {
		return nil, err
	}
	return strings.Split(string(data), "\n"), nil
}

// MakeHashTable creates a hash table of encrypted words to their
// unencrypted counterparts.
func MakeHashTable(wordlist []string) map[string]string {
	table := make(map[string]string, len(wordlist))
	for _, word := range wordlist {
		table[EncryptString(word)] = word
	}
	return table
}
