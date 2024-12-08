package main

import (
	"fmt"
	"log"
	"os"
	"strings"
	"strconv"
)

func main() {
	input := get_input()
	preamble_size, err := strconv.Atoi(os.Args[2])
	if err != nil {
		log.Fatal(err)
	}
	preamble := get_preamble(&input, preamble_size)
	fmt.Println(input)
	fmt.Println(preamble)
}

func get_input() []int {
	if len(os.Args) < 3 {
		log.Fatal("Usage: go run script1.go <filename> <preamble_size>")
	}
	content, err := os.ReadFile(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	var input []int
	for _, line := range strings.Split(string(content), "\n") {
		if line != "" {
			value, err := strconv.Atoi(line)
			if err != nil {
				log.Fatal(err)
			}
			input = append(input, value)
		}
	}
	return input
}

func get_preamble(input *[]int, preamble_size int) []int {
	// remove the preamble_size first elements from the input and return them
	preamble := (*input)[:preamble_size]
	*input = (*input)[preamble_size:]
	return preamble
}
