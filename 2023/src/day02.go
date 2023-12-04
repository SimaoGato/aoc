package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

const (
	RED_CUBES   = 12
	GREEN_CUBES = 13
	BLUE_CUBES  = 14
)

// CubeColorLimits defines the maximum allowed count for each cube color.
var CubeColorLimits = map[string]int{
	"red":   RED_CUBES,
	"green": GREEN_CUBES,
	"blue":  BLUE_CUBES,
}

// processGame checks if the count of cubes exceeds the limit for a given color.
func processGame(gameNum int, cubesSet string) bool {
	var redBool, greenBool, blueBool bool

	cubesSubset := strings.Split(cubesSet, ";")

	for _, totalCubes := range cubesSubset {
		colouredCubes := strings.Split(totalCubes, ",")

		for _, cube := range colouredCubes {
			cubeInfo := strings.Split(cube, " ")

			color, count := cubeInfo[2], cubeInfo[1]
			limit, exists := CubeColorLimits[color]

			if !exists {
				fmt.Println("Error: Unknown cube color")
				return false
			}

			countInt, err := strconv.Atoi(count)
			if err != nil {
				fmt.Println("Error: Invalid cube count")
				return false
			}

			switch color {
			case "red":
				if countInt > limit {
					redBool = true
					break
				}
			case "green":
				if countInt > limit {
					greenBool = true
					break
				}
			case "blue":
				if countInt > limit {
					blueBool = true
					break
				}
			}
		}

		if redBool || greenBool || blueBool {
			break
		}
	}

	return !(redBool || greenBool || blueBool)
}

func checkMinSetOfCubes(cubesSet string) int {

	var red_count, green_count, blue_count int = 0, 0, 0

	cubesSubset := strings.Split(cubesSet, ";")

	for _, totalCubes := range cubesSubset {

		colouredCubes := strings.Split(totalCubes, ",")

		for _, cube := range colouredCubes {
			cubeInfo := strings.Split(cube, " ")

			color, count := cubeInfo[2], cubeInfo[1]

			countInt, err := strconv.Atoi(count)
			if err != nil {
				fmt.Println("Error: Invalid cube count")
				return 0
			}

			switch color {
			case "red":
				if countInt > red_count {
					red_count = countInt
					break
				}
			case "green":
				if countInt > green_count {
					green_count = countInt
					break
				}
			case "blue":
				if countInt > blue_count {
					blue_count = countInt
					break
				}
			}
		}

	}

	return red_count * green_count * blue_count

}

func main() {
	filePath := "../inputs/day02.in"

	file, err := os.Open(filePath)
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var totalGameID int
	var minSetOfCubes int

	for scanner.Scan() {
		line := scanner.Text()
		aux := strings.Split(line, ":")
		gameStr := aux[0]
		gameSplit := strings.Split(gameStr, " ")
		gameNum, err := strconv.Atoi(gameSplit[1])
		if err != nil {
			fmt.Println("Error parsing game number:", err)
			return
		}

		cubesSet := aux[1]

		if processGame(gameNum, cubesSet) {
			totalGameID += gameNum
		}

		minSetOfCubes += checkMinSetOfCubes(cubesSet)

	}

	fmt.Printf("Total Game ID: %d\n", totalGameID)
	fmt.Printf("Sum Of The Power Of Minimal Sets: %d\n", minSetOfCubes)
}
