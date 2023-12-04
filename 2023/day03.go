package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	filePath := "inputs/day03.in"

	var matrix = doMatrix(filePath)

	var parts int = calculateEngineSchematic(matrix)

	var gearRatios int = calculateGearRatios(matrix)

	fmt.Println("The sum of all of the part numbers in the engine schematic is: ", parts)
	fmt.Println("The sum of all of the gear ratios are: ", gearRatios)
}

func doMatrix(filePath string) [][]string {
	file, err := os.Open(filePath)
	if err != nil {
		fmt.Println("Error opening file:", err)
		return nil
	}
	defer func(file *os.File) {
		err := file.Close()
		if err != nil {
			// Handle file close error if needed
		}
	}(file)

	scanner := bufio.NewScanner(file)
	var matrix [][]string

	for scanner.Scan() {
		line := scanner.Text()
		var row []string
		for _, char := range line {
			row = append(row, string(char))
		}
		matrix = append(matrix, row)
	}

	return matrix
}

func calculateGearRatios(matrix [][]string) int {
	var total int = 0

	for i := 0; i < len(matrix); i++ {
		for j := 0; j < len(matrix[i]); j++ {
			if matrix[i][j] == "*" {
				total += calculateGearRatio(matrix, i, j)
			}
		}
	}

	return total
}

func calculateGearRatio(matrix [][]string, i int, j int) int {
	var gearRatio int = 1
	var cols int = len(matrix[0])
	var rows int = len(matrix)
	var count int = 0
	var above bool = false
	var below bool = false

	// Check above
	if i > 0 && matrix[i-1][j] >= "0" && matrix[i-1][j] <= "9" {
		gearRatio *= checkForNumberAboveOrBelow(matrix, i-1, j)
		count++
		above = true
	}

	// Check top-left corner
	if !above && i > 0 && j > 0 && matrix[i-1][j-1] >= "0" && matrix[i-1][j-1] <= "9" {
		gearRatio *= checkForNumberTopLeft(matrix, i-1, j-1)
		count++
	}

	// Check top-right corner
	if !above && i > 0 && j < cols-1 && matrix[i-1][j+1] >= "0" && matrix[i-1][j+1] <= "9" {
		gearRatio *= checkForNumberTopRight(matrix, i-1, j+1)
		count++
	}

	// Check below
	if i < rows-1 && matrix[i+1][j] >= "0" && matrix[i+1][j] <= "9" {
		gearRatio *= checkForNumberAboveOrBelow(matrix, i+1, j)
		count++
		below = true
	}

	// Check bottom-left corner
	if !below && i < rows-1 && matrix[i+1][j-1] >= "0" && matrix[i+1][j-1] <= "9" {
		gearRatio *= checkForNumberTopLeft(matrix, i+1, j-1)
		count++
	}

	// Check bottom-right corner
	if !below && i < rows-1 && matrix[i+1][j+1] >= "0" && matrix[i+1][j+1] <= "9" {
		gearRatio *= checkForNumberTopRight(matrix, i+1, j+1)
		count++
	}

	// Check left
	if j > 0 && matrix[i][j-1] >= "0" && matrix[i][j-1] <= "9" {
		gearRatio *= checkForNumberTopLeft(matrix, i, j-1)
		count++
	}

	// Check right
	if j < cols-1 && matrix[i][j+1] >= "0" && matrix[i][j+1] <= "9" {
		gearRatio *= checkForNumberTopRight(matrix, i, j+1)
		count++
	}

	// return gearRatio if count == 2, return 0 if count != 2
	if count == 2 {
		return gearRatio
	} else {
		return 0
	}
}

func checkForNumberAboveOrBelow(matrix [][]string, i int, j int) int {
	var result string = matrix[i][j]
	var jRight int = j + 1
	var jLeft int = j - 1

	for jRight < len(matrix[i]) && matrix[i][jRight] >= "0" && matrix[i][jRight] <= "9" {
		result = result + matrix[i][jRight]
		jRight++
	}

	for jLeft >= 0 && matrix[i][jLeft] >= "0" && matrix[i][jLeft] <= "9" {
		result = matrix[i][jLeft] + result
		jLeft--
	}

	resultInt, err := strconv.Atoi(result)

	if err != nil {
		fmt.Println("Error: Invalid gear ratio")
		return 0
	}

	return resultInt
}

func checkForNumberTopLeft(matrix [][]string, i int, j int) int {
	var result string = matrix[i][j]
	var jLeft int = j - 1

	for jLeft >= 0 && matrix[i][jLeft] >= "0" && matrix[i][jLeft] <= "9" {
		result = matrix[i][jLeft] + result
		jLeft--
	}

	resultInt, err := strconv.Atoi(result)

	if err != nil {
		fmt.Println("Error: Invalid gear ratio")
		return 0
	}

	return resultInt
}

func checkForNumberTopRight(matrix [][]string, i int, j int) int {
	var result string = matrix[i][j]
	var jRight int = j + 1

	for jRight < len(matrix[i]) && matrix[i][jRight] >= "0" && matrix[i][jRight] <= "9" {
		result = result + matrix[i][jRight]
		jRight++
	}

	resultInt, err := strconv.Atoi(result)

	if err != nil {
		fmt.Println("Error: Invalid gear ratio")
		return 0
	}

	return resultInt
}

func calculateEngineSchematic(matrix [][]string) int {
	var belongsToPartNumber bool = false
	var tempPartNumber int = 0
	var isValidNumber bool = false
	var total int = 0

	for i := 0; i < len(matrix); i++ {
		for j := 0; j < len(matrix[i]); j++ {
			if matrix[i][j] >= "0" && matrix[i][j] <= "9" {
				if !belongsToPartNumber {
					tempPartNumber = int(matrix[i][j][0] - '0')
					belongsToPartNumber = true
				} else if belongsToPartNumber {
					tempPartNumber = tempPartNumber*10 + int(matrix[i][j][0]-'0')
				}
				if !isValidNumber {
					isValidNumber = checkValidity(matrix, i, j)
				}
			} else {
				if belongsToPartNumber && isValidNumber {
					total += tempPartNumber
				}
				belongsToPartNumber = false
				tempPartNumber = 0
				isValidNumber = false
			}
		}
	}

	return total
}

func checkValidity(matrix [][]string, i int, j int) bool {
	rows := len(matrix)
	cols := len(matrix[0])

	// Check if the number is valid
	// For this, the number should be adjacent to a symbol (except '.')
	// So, it has to either have a symbol above, below, left, right, or in the corners

	// Check above
	if i > 0 && matrix[i-1][j] != "." && (matrix[i-1][j] < "0" || matrix[i-1][j] > "9") {
		return true
	}

	// Check below
	if i < rows-1 && matrix[i+1][j] != "." && (matrix[i+1][j] < "0" || matrix[i+1][j] > "9") {
		return true
	}

	// Check left
	if j > 0 && matrix[i][j-1] != "." && (matrix[i][j-1] < "0" || matrix[i][j-1] > "9") {
		return true
	}

	// Check right
	if j < cols-1 && matrix[i][j+1] != "." && (matrix[i][j+1] < "0" || matrix[i][j+1] > "9") {
		return true
	}

	// Check top-left corner
	if i > 0 && j > 0 && matrix[i-1][j-1] != "." && (matrix[i-1][j-1] < "0" || matrix[i-1][j-1] > "9") {
		return true
	}

	// Check top-right corner
	if i > 0 && j < cols-1 && matrix[i-1][j+1] != "." && (matrix[i-1][j+1] < "0" || matrix[i-1][j+1] > "9") {
		return true
	}

	// Check bottom-left corner
	if i < rows-1 && j > 0 && matrix[i+1][j-1] != "." && (matrix[i+1][j-1] <= "0" || matrix[i+1][j-1] >= "9") {
		return true
	}

	// Check bottom-right corner
	if i < rows-1 && j < cols-1 && matrix[i+1][j+1] != "." && (matrix[i+1][j+1] <= "0" || matrix[i+1][j+1] >= "9") {
		return true
	}

	return false
}
