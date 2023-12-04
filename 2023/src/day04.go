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
	numberOfCopies  int
}

func main() {

	filePath := "../inputs/day04.in"

	var gameCards = doGameCards(filePath)

	var gameCardsPoints int = calculateGameCardsPoints(gameCards)
	var totalScratchCards int = calculateTotalScratchCards(gameCards)

	fmt.Println("The game cards are worth in total: ", gameCardsPoints)
	fmt.Println("The total number of scratch cards is: ", totalScratchCards)
}

func doGameCards(filePath string) []GameCard {
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

		gameCard := GameCard{cardNum, winningNumbers, selectedNumbers, 1}
		gameCards = append(gameCards, gameCard)
	}

	return gameCards
}

func calculateTotalScratchCards(gameCards []GameCard) int {

	for i := 0; i < len(gameCards); i++ {
		numberOfMatches := calculateNumberOfMatches(gameCards[i])

		if numberOfMatches != 0 {
			for k := 1; k <= gameCards[i].numberOfCopies; k++ {
				for j := i + 1; j <= i+numberOfMatches; j++ {
					gameCards[j].numberOfCopies++
				}
			}
		}

	}

	return calculateTotalScratchCardsAux(gameCards)
}

func calculateTotalScratchCardsAux(gameCards []GameCard) int {
	var totalScratchCards int = 0

	for _, gameCard := range gameCards {
		totalScratchCards += gameCard.numberOfCopies
	}

	return totalScratchCards
}

func calculateNumberOfMatches(gameCard GameCard) int {

	var numberOfMatches int = 0

	for _, selectedNumber := range gameCard.SelectedNumbers {
		if contains(gameCard.WinningNumbers, selectedNumber) {
			numberOfMatches++
		}
	}

	return numberOfMatches
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
