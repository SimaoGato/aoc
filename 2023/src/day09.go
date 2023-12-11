package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	// Parse input
	filePath := "../inputs/day09.in"

	// Day 09 - Part 1
	extrapolatedValuesSum := calculatePosteriorExtrapolatedValuesSum(filePath)

	fmt.Println("Extrapolated last values sum is: ", extrapolatedValuesSum)

	// Day 09 - Part 2
	extrapolatedValuesSum = calculateAnteriorExtrapolatedValuesSum(filePath)

	fmt.Println("Extrapolated first values sum is: ", extrapolatedValuesSum)
}

func calculateAnteriorExtrapolatedValuesSum(filePath string) int {
	file, err := os.Open(filePath)
	if err != nil {
		fmt.Println("Error opening the file:", err)
		return 0
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	extrapolatedValuesSum := 0

	for scanner.Scan() {
		sequence := scanner.Text()
		extrapolatedValuesSum += calculateAnteriorSequenceValue(sequence)
	}

	return extrapolatedValuesSum
}

func calculateAnteriorSequenceValue(sequence string) int {
	intValue := 0
	sequenceCalculationStack := make([][]int, 1)

	strValues := strings.Fields(sequence)

	for _, strValue := range strValues {
		intValue, _ = strconv.Atoi(strValue)
		sequenceCalculationStack[0] = append(sequenceCalculationStack[0], intValue)
	}

	for !lastLineZeros(sequenceCalculationStack[len(sequenceCalculationStack)-1]) {
		sequenceCalculationStack = append(sequenceCalculationStack, make([]int, 0))

		for i := 0; i < len(sequenceCalculationStack[len(sequenceCalculationStack)-2])-1; i++ {
			valueToPut := sequenceCalculationStack[len(sequenceCalculationStack)-2][i+1] - sequenceCalculationStack[len(sequenceCalculationStack)-2][i]
			sequenceCalculationStack[len(sequenceCalculationStack)-1] = append(sequenceCalculationStack[len(sequenceCalculationStack)-1], valueToPut)
		}
	}

	return calculateAnteriorValue(sequenceCalculationStack)
}

func calculateAnteriorValue(sequenceCalculationStack [][]int) int {
	for i := 0; i < len(sequenceCalculationStack)-1; i++ {
		idx := len(sequenceCalculationStack) - i - 1
		minuendIdx := 0
		subtrahendIdx := 0
		valueToPut := sequenceCalculationStack[idx-1][minuendIdx] - sequenceCalculationStack[idx][subtrahendIdx]
		// append to the beginning of the slice
		sequenceCalculationStack[idx-1] = append([]int{valueToPut}, sequenceCalculationStack[idx-1]...)
	}

	return sequenceCalculationStack[0][0]
}

func calculatePosteriorExtrapolatedValuesSum(filePath string) int {
	file, err := os.Open(filePath)
	if err != nil {
		fmt.Println("Error opening the file:", err)
		return 0
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	extrapolatedValuesSum := 0

	for scanner.Scan() {
		sequence := scanner.Text()
		extrapolatedValuesSum += calculateSequenceNextValue(sequence)
	}

	return extrapolatedValuesSum
}

func calculateSequenceNextValue(sequence string) int {
	intValue := 0
	sequenceCalculationStack := make([][]int, 1)

	strValues := strings.Fields(sequence)

	for _, strValue := range strValues {
		intValue, _ = strconv.Atoi(strValue)
		sequenceCalculationStack[0] = append(sequenceCalculationStack[0], intValue)
	}

	for !lastLineZeros(sequenceCalculationStack[len(sequenceCalculationStack)-1]) {
		sequenceCalculationStack = append(sequenceCalculationStack, make([]int, 0))

		for i := 0; i < len(sequenceCalculationStack[len(sequenceCalculationStack)-2])-1; i++ {
			valueToPut := sequenceCalculationStack[len(sequenceCalculationStack)-2][i+1] - sequenceCalculationStack[len(sequenceCalculationStack)-2][i]
			sequenceCalculationStack[len(sequenceCalculationStack)-1] = append(sequenceCalculationStack[len(sequenceCalculationStack)-1], valueToPut)
		}
	}

	return calculateNextValue(sequenceCalculationStack)
}

func calculateNextValue(sequenceCalculationStack [][]int) int {
	for i := 0; i < len(sequenceCalculationStack)-1; i++ {
		idx := len(sequenceCalculationStack) - i - 1
		minuendIdx := len(sequenceCalculationStack[idx-1]) - 1
		subtrahendIdx := len(sequenceCalculationStack[idx]) - 1
		valueToPut := sequenceCalculationStack[idx-1][minuendIdx] + sequenceCalculationStack[idx][subtrahendIdx]
		sequenceCalculationStack[idx-1] = append(sequenceCalculationStack[idx-1], valueToPut)
	}

	return sequenceCalculationStack[0][len(sequenceCalculationStack[0])-1]
}

func lastLineZeros(lastLine []int) bool {
	for i := 0; i < len(lastLine); i++ {
		if lastLine[i] != 0 {
			return false
		}
	}

	return true
}
