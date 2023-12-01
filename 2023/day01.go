package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
	file, err := os.Open("inputs/day01.in")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	var totalCalibration int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		// Calculate the calibration value for each line
		calibration := 0
		found_first_digit := false
		found_last_digit := false
		first_digit := 0
		last_digit := 0

		for i := 0; i < len(line); i++ {

			// Check if line[i] is a digit
			if line[i] >= '0' && line[i] <= '9' && !found_first_digit {
				// Convert line[i] to int
				first_digit = int(line[i] - '0')
				found_first_digit = true
			}

			// Check if line[len(line)-i] is a digit
			if line[len(line)-i-1] >= '0' && line[len(line)-i-1] <= '9' && !found_last_digit {
				// Convert line[len(line)-i] to int
				last_digit = int(line[len(line)-i-1] - '0')
				found_last_digit = true
			}

			// If both digits are found, break out of the loop
			if found_first_digit && found_last_digit {
				break
			}

		}
		
		calibration = first_digit * 10 + last_digit

		fmt.Printf("Calibration for line \"%s\": %d\n", line, calibration)
		totalCalibration += calibration
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Total Calibration: %d\n", totalCalibration)
}
