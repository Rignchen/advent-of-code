package main

import (
	"fmt"
	"log"
	"os"
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
	for _, line := range string(content) {
		input = append(input, int(line))
	}
	return input
}
