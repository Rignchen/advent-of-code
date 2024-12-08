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
	fmt.Println(input)
}

func get_input() []int {
	if len(os.Args) < 2 {
		log.Fatal("Usage: go run script1.go <filename>")
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
