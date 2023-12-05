package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type seedMap struct {
	soil        int
	fertilizer  int
	water       int
	light       int
	temperature int
	humidity    int
	location    int
}

func main() {
	filePath := "../inputs/day05.in"

	var seedsToPlant []int
	var seedsMap map[int]seedMap = make(map[int]seedMap)

	seedsToPlant = parseInput(filePath, &seedsMap)

	minLocation := checkMinimumLocation(seedsToPlant, &seedsMap)

	/*for _, seed := range seedsToPlant {
		fmt.Println("Seed: ", seed, " Soil: ", seedsMap[seed].soil, " Fertilizer: ", seedsMap[seed].fertilizer, " Water: ", seedsMap[seed].water, " Light: ", seedsMap[seed].light, " Temperature: ", seedsMap[seed].temperature, " Humidity: ", seedsMap[seed].humidity, " Location: ", seedsMap[seed].location)
	}*/

	fmt.Println(minLocation)
}

func parseInput(filePath string, seedsMap *map[int]seedMap) []int {
	var seedsToPlant []int

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

	var blankLine int = 0

	for scanner.Scan() {
		line := scanner.Text()

		if len(line) == 0 {
			blankLine++
		} else if (line[0] >= '0' && line[0] <= '9') || (blankLine == 0 && line[0:5] == "seeds") {
			switch blankLine {
			case 0:
				seedsToPlant = parseSeedsToPlant(line, seedsMap)
				break
			case 1:
				parseSeedToSoil(line, seedsMap, seedsToPlant)
				break
			case 2:
				parseSoilToFertilizer(line, seedsMap, seedsToPlant)
				break
			case 3:
				parseFertilizerToWater(line, seedsMap, seedsToPlant)
				break
			case 4:
				parseWaterToLight(line, seedsMap, seedsToPlant)
				break
			case 5:
				parseLightToTemperature(line, seedsMap, seedsToPlant)
				break
			case 6:
				parseTemperatureToHumidity(line, seedsMap, seedsToPlant)
				break
			case 7:
				parseHumidityToLocation(line, seedsMap, seedsToPlant)
				break
			default:
				break
			}
		} else {
			continue
		}
	}

	return seedsToPlant
}

func parseSeedsToPlant(line string, seedsMap *map[int]seedMap) []int {
	var seedsToPlant []int
	var i int = 1

	info := strings.Split(line, " ")

	for i < len(info) {
		seed, _ := strconv.Atoi(info[i])
		seedsToPlant = append(seedsToPlant, seed)
		(*seedsMap)[seed] = seedMap{seed, seed, seed, seed, seed, seed, seed}
		i++
	}

	return seedsToPlant
}

func parseSeedToSoil(line string, seedsMap *map[int]seedMap, seedsToPlant []int) {
	info := strings.Split(line, " ")

	destRangeStart, _ := strconv.Atoi(info[0])
	sourceRangeStart, _ := strconv.Atoi(info[1])
	rangeLength, _ := strconv.Atoi(info[2])

	for i := 0; i < len(seedsToPlant); i++ {
		if seedsToPlant[i] >= sourceRangeStart && seedsToPlant[i] < sourceRangeStart+rangeLength {
			calc := destRangeStart + seedsToPlant[i] - sourceRangeStart
			(*seedsMap)[seedsToPlant[i]] = seedMap{calc, calc, calc, calc, calc, calc, calc}
		}
	}
}

func parseSoilToFertilizer(line string, seedsMap *map[int]seedMap, seedsToPlant []int) {
	info := strings.Split(line, " ")

	destRangeStart, _ := strconv.Atoi(info[0])
	sourceRangeStart, _ := strconv.Atoi(info[1])
	rangeLength, _ := strconv.Atoi(info[2])

	for i := 0; i < len(seedsToPlant); i++ {
		if (*seedsMap)[seedsToPlant[i]].soil >= sourceRangeStart && (*seedsMap)[seedsToPlant[i]].soil < sourceRangeStart+rangeLength {
			calc := destRangeStart + (*seedsMap)[seedsToPlant[i]].soil - sourceRangeStart
			(*seedsMap)[seedsToPlant[i]] = seedMap{(*seedsMap)[seedsToPlant[i]].soil, calc, calc, calc, calc, calc, calc}
		}
	}
}

func parseFertilizerToWater(line string, seedsMap *map[int]seedMap, seedsToPlant []int) {
	info := strings.Split(line, " ")

	destRangeStart, _ := strconv.Atoi(info[0])
	sourceRangeStart, _ := strconv.Atoi(info[1])
	rangeLength, _ := strconv.Atoi(info[2])

	for i := 0; i < len(seedsToPlant); i++ {
		if (*seedsMap)[seedsToPlant[i]].fertilizer >= sourceRangeStart && (*seedsMap)[seedsToPlant[i]].fertilizer < sourceRangeStart+rangeLength {
			calc := destRangeStart + (*seedsMap)[seedsToPlant[i]].fertilizer - sourceRangeStart
			(*seedsMap)[seedsToPlant[i]] = seedMap{(*seedsMap)[seedsToPlant[i]].soil, (*seedsMap)[seedsToPlant[i]].fertilizer, calc, calc, calc, calc, calc}
		}
	}
}

func parseWaterToLight(line string, seedsMap *map[int]seedMap, seedsToPlant []int) {
	info := strings.Split(line, " ")

	destRangeStart, _ := strconv.Atoi(info[0])
	sourceRangeStart, _ := strconv.Atoi(info[1])
	rangeLength, _ := strconv.Atoi(info[2])

	for i := 0; i < len(seedsToPlant); i++ {
		if (*seedsMap)[seedsToPlant[i]].water >= sourceRangeStart && (*seedsMap)[seedsToPlant[i]].water < sourceRangeStart+rangeLength {
			calc := destRangeStart + (*seedsMap)[seedsToPlant[i]].water - sourceRangeStart
			(*seedsMap)[seedsToPlant[i]] = seedMap{(*seedsMap)[seedsToPlant[i]].soil, (*seedsMap)[seedsToPlant[i]].fertilizer, (*seedsMap)[seedsToPlant[i]].water, calc, calc, calc, calc}
		}
	}
}

func parseLightToTemperature(line string, seedsMap *map[int]seedMap, seedsToPlant []int) {
	info := strings.Split(line, " ")

	destRangeStart, _ := strconv.Atoi(info[0])
	sourceRangeStart, _ := strconv.Atoi(info[1])
	rangeLength, _ := strconv.Atoi(info[2])

	for i := 0; i < len(seedsToPlant); i++ {
		if (*seedsMap)[seedsToPlant[i]].light >= sourceRangeStart && (*seedsMap)[seedsToPlant[i]].light < sourceRangeStart+rangeLength {
			calc := destRangeStart + (*seedsMap)[seedsToPlant[i]].light - sourceRangeStart
			(*seedsMap)[seedsToPlant[i]] = seedMap{(*seedsMap)[seedsToPlant[i]].soil, (*seedsMap)[seedsToPlant[i]].fertilizer, (*seedsMap)[seedsToPlant[i]].water, (*seedsMap)[seedsToPlant[i]].light, calc, calc, calc}
		}
	}
}

func parseTemperatureToHumidity(line string, seedsMap *map[int]seedMap, seedsToPlant []int) {
	info := strings.Split(line, " ")

	destRangeStart, _ := strconv.Atoi(info[0])
	sourceRangeStart, _ := strconv.Atoi(info[1])
	rangeLength, _ := strconv.Atoi(info[2])

	for i := 0; i < len(seedsToPlant); i++ {
		if (*seedsMap)[seedsToPlant[i]].temperature >= sourceRangeStart && (*seedsMap)[seedsToPlant[i]].temperature < sourceRangeStart+rangeLength {
			calc := destRangeStart + (*seedsMap)[seedsToPlant[i]].temperature - sourceRangeStart
			(*seedsMap)[seedsToPlant[i]] = seedMap{(*seedsMap)[seedsToPlant[i]].soil, (*seedsMap)[seedsToPlant[i]].fertilizer, (*seedsMap)[seedsToPlant[i]].water, (*seedsMap)[seedsToPlant[i]].light, (*seedsMap)[seedsToPlant[i]].temperature, calc, calc}
		}
	}
}

func parseHumidityToLocation(line string, seedsMap *map[int]seedMap, seedsToPlant []int) {
	info := strings.Split(line, " ")

	destRangeStart, _ := strconv.Atoi(info[0])
	sourceRangeStart, _ := strconv.Atoi(info[1])
	rangeLength, _ := strconv.Atoi(info[2])

	for i := 0; i < len(seedsToPlant); i++ {
		if (*seedsMap)[seedsToPlant[i]].humidity >= sourceRangeStart && (*seedsMap)[seedsToPlant[i]].humidity < sourceRangeStart+rangeLength {
			calc := destRangeStart + (*seedsMap)[seedsToPlant[i]].humidity - sourceRangeStart
			(*seedsMap)[seedsToPlant[i]] = seedMap{(*seedsMap)[seedsToPlant[i]].soil, (*seedsMap)[seedsToPlant[i]].fertilizer, (*seedsMap)[seedsToPlant[i]].water, (*seedsMap)[seedsToPlant[i]].light, (*seedsMap)[seedsToPlant[i]].temperature, (*seedsMap)[seedsToPlant[i]].humidity, calc}
		}
	}
}

func checkMinimumLocation(seedsToPlant []int, seedsMap *map[int]seedMap) int {
	var minLocation int = -1

	for _, seed := range seedsToPlant {
		if minLocation == -1 {
			minLocation = (*seedsMap)[seed].location
		} else if (*seedsMap)[seed].location < minLocation {
			minLocation = (*seedsMap)[seed].location
		}
	}

	return minLocation
}
