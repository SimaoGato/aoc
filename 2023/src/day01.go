package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	filePath := "inputs/day01.in"
	totalCalibration, err := calculateTotalCalibration(filePath)
	if err != nil {
		log.Fatal(err)
	}

	totalImprovedCalibration, err := calculateTotalImprovedCalibration(filePath)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Total Calibration: %d\n", totalCalibration)
	fmt.Printf("Total Improved Calibration: %d\n", totalImprovedCalibration)
}

func calculateTotalImprovedCalibration(filePath string) (int, error) {
	file, err := os.Open(filePath)
	if err != nil {
		return 0, err
	}
	defer file.Close()

	var totalCalibration int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		calibration, err := calculateImprovedCalibration(line)
		if err != nil {
			return 0, err
		}

		totalCalibration += calibration
	}

	if err := scanner.Err(); err != nil {
		return 0, err
	}

	return totalCalibration, nil
}

func calculateImprovedCalibration(line string) (int, error) {
	line = replaceNumbersWithDigits(line)

	calibration, _ := calculateCalibration(line)

	return calibration, nil
}

func replaceNumbersWithDigits(input string) string {
	replacements := map[string]string{
		"one":   "o1e",
		"two":   "t2o",
		"three": "t3e",
		"four":  "4",
		"five":  "5e",
		"six":   "6",
		"seven": "7n",
		"eight": "e8t",
		"nine":  "n9e",
	}

	for i := 0; i < len(input); i++ {
		if input[i] >= 'a' && input[i] <= 'z' {
			for key, value := range replacements {
				if strings.HasPrefix(input[i:], key) {
					input = input[:i] + value + input[i+len(key):]
				}
			}
		}
	}

	return input
}

func calculateTotalCalibration(filePath string) (int, error) {
	file, err := os.Open(filePath)
	if err != nil {
		return 0, err
	}
	defer file.Close()

	var totalCalibration int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		calibration, err := calculateCalibration(line)
		if err != nil {
			return 0, err
		}

		totalCalibration += calibration
	}

	if err := scanner.Err(); err != nil {
		return 0, err
	}

	return totalCalibration, nil
}

func calculateCalibration(line string) (int, error) {
	var firstDigit, lastDigit int

	for i := 0; i < len(line); i++ {
		if line[i] >= '0' && line[i] <= '9' {
			firstDigit = int(line[i] - '0')
			break
		}
	}

	for i := len(line) - 1; i >= 0; i-- {
		if line[i] >= '0' && line[i] <= '9' {
			lastDigit = int(line[i] - '0')
			break
		}
	}

	return firstDigit*10 + lastDigit, nil
}
