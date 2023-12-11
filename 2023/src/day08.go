package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	filePath := "../inputs/day08.in"

	file, err := os.Open(filePath)
	if err != nil {
		fmt.Println("Error opening the file:", err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	// Part 1
	instructions, network := parseMapInput(scanner)

	steps := getSteps(network, instructions)

	fmt.Println("Steps required to reach ZZZ are: ", steps)

	// Part 2
	initialPositions := getInitialPositionsList(network)

	everySteps := getUpdatedSteps(network, instructions, initialPositions)

	fmt.Println("Steps required to reach ZZZ with multiple initial positions are: ", lcmAll(everySteps[0], everySteps[1:]...))
}

func parseMapInput(scanner *bufio.Scanner) (string, map[string][]string) {
	var instructions string
	network := make(map[string][]string)

	scanner.Scan()
	instructions = scanner.Text()

	for scanner.Scan() {
		// if line is not blank
		if len(scanner.Text()) > 0 {
			line := scanner.Text()
			info := strings.Split(line, "=")

			origin := strings.TrimSpace(info[0])
			info = strings.Split(info[1], ",")

			destLeft := strings.TrimSpace(info[0])
			destRight := strings.TrimSpace(info[1])

			destLeft = destLeft[1:]
			destRight = destRight[0 : len(destRight)-1]

			network[origin] = append(network[origin], destLeft)
			network[origin] = append(network[origin], destRight)
		}
	}

	return instructions, network
}

func getInitialPositionsList(network map[string][]string) []string {
	var initialPositions []string

	for key := range network {
		if key[2] == 'A' {
			initialPositions = append(initialPositions, key)
		}
	}

	return initialPositions
}

func getSteps(network map[string][]string, instructions string) int {
	steps := 0
	currentPos := "AAA"
	instructionsLen := len(instructions)

	for currentPos != "ZZZ" {
		// find the next possible steps
		possibleSteps := network[currentPos]

		if instructions[steps%instructionsLen] == 'L' {
			currentPos = possibleSteps[0]
		} else {
			currentPos = possibleSteps[1]
		}

		steps++
	}

	return steps
}

func getUpdatedSteps(network map[string][]string, instructions string, initialPositions []string) []int {
	everySteps := make([]int, len(initialPositions))

	for i, initialPos := range initialPositions {
		everySteps[i] = getUpdatedStepsAux(network, instructions, initialPos)
	}

	return everySteps
}

func getUpdatedStepsAux(network map[string][]string, instructions string, initialPos string) int {
	steps := 0
	currentPos := initialPos
	instructionsLen := len(instructions)

	for currentPos[2] != 'Z' {
		// find the next possible steps
		possibleSteps := network[currentPos]

		if instructions[steps%instructionsLen] == 'L' {
			currentPos = possibleSteps[0]
		} else {
			currentPos = possibleSteps[1]
		}

		steps++
	}

	return steps
}

// gcd,lcm,lcmAll inspired by:
// https://stackoverflow.com/questions/31302054/how-to-find-the-least-common-multiple-of-a-range-of-numbers
func gcd(a, b int) int {
	if b == 0 {
		return a
	}
	return gcd(b, a%b)
}
func lcm(a, b int) int {
	return a / gcd(a, b) * b
}
func lcmAll(a int, bs ...int) int {
	result := a
	for _, b := range bs {
		result = lcm(result, b)
	}

	return result
}
