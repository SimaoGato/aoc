package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	// Open the file
	file, err := os.Open("../inputs/day06.in")
	if err != nil {
		fmt.Println("Error opening the file:", err)
		return
	}
	defer file.Close()

	// Create a scanner to read the file line by line
	scanner := bufio.NewScanner(file)

	// Initialize a map to store CorrelatedData instances
	dataMap := make(map[int]int)

	// Parse input to dateMap
	parseInputToDataMap(scanner, &dataMap)

	winWays := calculateWinWays(dataMap)

	fmt.Println("Winning ways multiplied together: ", winWays)
}

func parseInputToDataMap(scanner *bufio.Scanner, dataMap *map[int]int) {
	scanner.Scan()
	timeLine := scanner.Text()
	scanner.Scan()
	distanceLine := scanner.Text()

	times := strings.Fields(timeLine)

	distances := strings.Fields(distanceLine)

	for i := 0; i < len(times); i++ {
		time, _ := strconv.Atoi(times[i])
		distance, _ := strconv.Atoi(distances[i])
		if time > 0 && distance > 0 {
			(*dataMap)[time] = distance
		}
	}
}

func calculateWinWays(dataMap map[int]int) int {
	var winWays int = 1

	for time, distance := range dataMap {
		countWinWays := 0

		for holdButton := 0; holdButton <= time; holdButton++ {
			timeLeft := time - holdButton
			velocity := holdButton

			if timeLeft*velocity > distance {
				countWinWays++
			}
		}

		winWays *= countWinWays
	}

	return winWays
}
