package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	filePath := "../inputs/day06.in"

	file, err := os.Open(filePath)
	if err != nil {
		fmt.Println("Error opening the file:", err)
		return
	}
	defer file.Close()

	// Create a scanner to read the file line by line
	scanner := bufio.NewScanner(file)

	scanner.Scan()
	timeLine := scanner.Text()
	scanner.Scan()
	distanceLine := scanner.Text()

	// Initialize a map to store CorrelatedData instances
	dataMap := make(map[int]int)

	// Parse input to dateMap
	parseInputToDataMap(&dataMap, timeLine, distanceLine)

	winWays := calculateWinWays(dataMap)

	fmt.Println("Winning ways multiplied together are: ", winWays)

	// Parse input the correct way
	correctTime, correctDistance := parseInputToCorrectData(timeLine, distanceLine)

	winWays = calculateWinWaysCorrect(correctTime, correctDistance)

	fmt.Println("Winning ways multiplied together, the correct way, are: ", winWays)
}

func parseInputToCorrectData(timeLine string, distanceLine string) (int, int) {

	correctTime := 0
	correctDistance := 0

	for i := 0; i < len(timeLine); i++ {
		if timeLine[i] >= '0' && timeLine[i] <= '9' {
			correctTime = correctTime*10 + int(timeLine[i]-'0')
		}
	}

	for i := 0; i < len(distanceLine); i++ {
		if distanceLine[i] >= '0' && distanceLine[i] <= '9' {
			correctDistance = correctDistance*10 + int(distanceLine[i]-'0')
		}
	}

	return correctTime, correctDistance
}

func calculateWinWaysCorrect(correctTime int, correctDistance int) int {
	var winWays int = 1

	countWinWays := 0

	for holdButton := 0; holdButton <= correctTime; holdButton++ {
		timeLeft := correctTime - holdButton
		velocity := holdButton

		if timeLeft*velocity > correctDistance {
			countWinWays++
		}
	}

	winWays *= countWinWays

	return winWays
}

func parseInputToDataMap(dataMap *map[int]int, timeLine string, distanceLine string) {
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
