package main

import (
	"fmt"
	"log"
	"os"
	"strings"
	"strconv"
	"errors"
)

func main() {
	input := get_input()
	copy_input := make([]int, len(input))
	copy(copy_input, input)
	preamble_size, err := strconv.Atoi(os.Args[2])
	if err != nil {
		log.Fatal(err)
	}
	preamble := get_preamble(&input, preamble_size)
	weakness := 0
	for _, value := range input {
		if !is_sum_of_two(&preamble, value) {
			weakness = value
			break
		}
		preamble = append(preamble[1:], value)
	}
	sum_range, err := find_sum_range(&copy_input, weakness)
	if err != nil {
		log.Fatal(err)
	}
	min := sum_range[0]
	max := sum_range[0]
	for _, value := range sum_range {
		if value < min {
			min = value
		}
		if value > max {
			max = value
		}
	}
	fmt.Println(min + max)
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

func is_sum_of_two(preamble *[]int, value int) bool {
	// check if the value is the sum of two numbers in the preamble
	for i, x := range *preamble {
		for _, y := range (*preamble)[i+1:] {
			if x + y == value {
				return true
			}
		}
	}
	return false
}

func sum_from_to(input *[]int, from int, to int) int {
	// sum the elements from index from to index to in the input
	sum := 0
	for _, value := range (*input)[from:to] {
		sum += value
	}
	return sum
}

func find_sum_range(input *[]int, value int) ([]int, error) {
	// find a contiguous set of at least two numbers in the input that sum to value
	for i := 0; i < len(*input); i++ {
		for j := i + 1; j < len(*input); j++ {
			if sum_from_to(input, i, j) == value {
				return (*input)[i:j], nil
			}
		}
	}
	return nil, errors.New("No contiguous set found")
}
