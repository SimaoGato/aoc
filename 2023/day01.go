package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
	filePath := "inputs/day01.in"
	totalCalibration, err := challenge1(filePath)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Total Calibration: %d\n", totalCalibration)
}

func challenge1(filePath string) (int, error) {
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

		fmt.Printf("Calibration for line \"%s\": %d\n", line, calibration)
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
