package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	filePath := "inputs/day03.in"

	file, err := os.Open(filePath)
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer func(file *os.File) {
		err := file.Close()
		if err != nil {
			// Handle file close error if needed
		}
	}(file)

	var matrix [][]string

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		var row []string
		for _, char := range line {
			row = append(row, string(char))
		}
		matrix = append(matrix, row)
	}

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

	fmt.Println(total)
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
