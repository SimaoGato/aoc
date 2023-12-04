package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type GameCard struct {
	CardNum         int
	WinningNumbers  []int
	SelectedNumbers []int
}

func main() {

	filePath := "../inputs/day04.in"

	var gameCardsPoints = doGameCards(filePath)

	fmt.Println(gameCardsPoints)
}

func doGameCards(filePath string) int {
	file, err := os.Open(filePath)
	if err != nil {
		fmt.Println("Error opening file:", err)
		return 0
	}
	defer func(file *os.File) {
		err := file.Close()
		if err != nil {
			// Handle file close error if needed
		}
	}(file)

	scanner := bufio.NewScanner(file)

	var gameCards []GameCard

	for scanner.Scan() {
		line := scanner.Text()
		info := strings.Split(line, ":")

		cardInfo := strings.Split(info[0], " ")
		cardNum, _ := strconv.Atoi(cardInfo[1])

		numbersInfo := strings.Split(info[1], "|")

		winningNumbersString := strings.Split(numbersInfo[0], " ")
		winningNumbers := parseToListOfInts(winningNumbersString)

		selectedNumbersString := strings.Split(numbersInfo[1], " ")
		selectedNumbers := parseToListOfInts(selectedNumbersString)

		gameCard := GameCard{cardNum, winningNumbers, selectedNumbers}
		gameCards = append(gameCards, gameCard)
	}

	gameCardsPoints := calculateGameCardsPoints(gameCards)

	return gameCardsPoints
}

func calculateGameCardsPoints(gameCards []GameCard) int {
	var gameCardsPoints int = 0

	for _, gameCard := range gameCards {
		var cardPoints int = calculateGameCardPoints(gameCard)
		gameCardsPoints += cardPoints
	}

	return gameCardsPoints
}

func calculateGameCardPoints(gameCard GameCard) int {
	var cardPoints int = 0

	for _, selectedNumber := range gameCard.SelectedNumbers {
		if contains(gameCard.WinningNumbers, selectedNumber) {
			if cardPoints == 0 {
				cardPoints = 1
			} else {
				cardPoints *= 2
			}
		}
	}

	return cardPoints
}

func contains(numbers []int, number int) bool {
	for _, num := range numbers {
		if num == number {
			return true
		}
	}

	return false
}

func parseToListOfInts(numbers []string) []int {
	var numbersInt []int

	for _, number := range numbers {
		if number == "" {
			continue
		} else {
			numberInt, _ := strconv.Atoi(number)
			numbersInt = append(numbersInt, numberInt)
		}
	}

	return numbersInt
}
