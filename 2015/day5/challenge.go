package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

func hasEnoughVowels(input string) bool {

	// count the number of vowels in the string
	var vowelCount int = 0

	for _, char := range input {
		if char == 'a' || char == 'e' || char == 'i' || char == 'o' || char == 'u' {
			vowelCount++
		}
		if vowelCount >= 3 {
			return true
		}
	}

	// return true if there are at least three vowels
	return false

}

func hasRepeatingLetter(input string) bool {

	// go through each letter and check if the next letter is the same
	for i := 0; i < len(input)-1; i++ {
		if input[i] == input[i+1] {
			return true
		}
	}

	// return true if there are at least three vowels
	return false

}

func hasForbiddenStrings(input string) bool {

	// check if the input contains any of the following strings: ab, cd, pq, or xy
	if strings.Contains(input, "ab") || strings.Contains(input, "cd") || strings.Contains(input, "pq") || strings.Contains(input, "xy") {
		return true
	}

	// return true if there are at least three vowels
	return false

}

func countNiceStrings(input string) int {
	var count int = 0

	// split the input into lines
	lines := strings.Split(input, "\n")

	// go through each line and check if it is nice
	for _, line := range lines {
		if hasEnoughVowels(line) && hasRepeatingLetter(line) && !hasForbiddenStrings(line) {
			count++
		}
	}

	return count
}

func hasNonOverlappingPair(input string) bool {

	for i := 0; i < len(input)-1; i++ {
		pair := input[i : i+2]
		if strings.Count(input, pair) >= 2 {
			return true
		}
	}

	return false

}

func hasRepeatingLetterWithOneBetween(input string) bool {

	// go through each letter and check if the next letter is the same
	for i := 0; i < len(input)-2; i++ {
		if input[i] == input[i+2] {
			return true
		}
	}

	// return true if there are at least three vowels
	return false

}

func countNiceStringsUpdated(input string) int {
	var count int = 0

	// split the input into lines
	lines := strings.Split(input, "\n")

	// go through each line and check if it is nice
	for _, line := range lines {
		if hasNonOverlappingPair(line) && hasRepeatingLetterWithOneBetween(line) {
			count++
		}
	}

	return count
}

func main() {
	// read the input file
	content, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	// go through each line and check if it is nice, return the number of nice strings
	var niceStrings int

	// niceStrings = countNiceStrings(string(content))
	niceStrings = countNiceStringsUpdated(string(content))

	fmt.Println("The number of nice strings is:", niceStrings)
}
