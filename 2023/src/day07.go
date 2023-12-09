package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

const (
	HIGHCARD     = 0
	ONEPAIR      = 1
	TWOPAIR      = 2
	THREEOFAKIND = 3
	FULLHOUSE    = 4
	FOUROFAKIND  = 5
	FIVEOFAKIND  = 6
)

var CardHits = map[string]int{
	"A": 13, // A has the highest value
	"K": 12,
	"Q": 11,
	"J": 10,
	"T": 9,
	"9": 8,
	"8": 7,
	"7": 6,
	"6": 5,
	"5": 4,
	"4": 3,
	"3": 2,
	"2": 1, // 2 has the lowest value
}

func main() {
	filePath := "../inputs/day07.in"

	file, err := os.Open(filePath)
	if err != nil {
		fmt.Println("Error opening the file:", err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	hands, bets := parseGameInput(scanner)

	bubbleSort(hands, bets)

	totalWinnings := calculateTotalWinnings(bets)

	fmt.Println("Total winnings are: ", totalWinnings)
}

func parseGameInput(scanner *bufio.Scanner) ([]string, []int) {
	var hands []string
	var bets []int

	for scanner.Scan() {
		line := scanner.Text()
		info := strings.Split(line, " ")

		hands = append(hands, info[0])
		bet, _ := strconv.Atoi(info[1])
		bets = append(bets, bet)
	}

	return hands, bets
}

func bubbleSort(hands []string, bets []int) {
	for i := 0; i < len(hands)-1; i++ {
		for j := 0; j < len(hands)-i-1; j++ {
			if isHand1BetterThanHand2(hands[j], hands[j+1]) {
				bets[j], bets[j+1] = bets[j+1], bets[j]
				hands[j], hands[j+1] = hands[j+1], hands[j]
			}
		}
	}
}

func isHand1BetterThanHand2(hand1 string, hand2 string) bool {
	hand1Type := getHandType(hand1)
	hand2Type := getHandType(hand2)

	if hand1Type > hand2Type {
		return true
	} else if hand1Type < hand2Type {
		return false
	} else {
		return isHand1BetterThanHand2Aux(hand1, hand2)
	}
}

func isHand1BetterThanHand2Aux(hand1 string, hand2 string) bool {
	for i := 0; i < 5; i++ {
		if CardHits[string(hand1[i])] > CardHits[string(hand2[i])] {
			return true
		} else if CardHits[string(hand1[i])] < CardHits[string(hand2[i])] {
			return false
		}
	}

	return false
}

func getHandType(hand string) int {
	var handType int = HIGHCARD

	if isFiveOfAKind(hand) {
		handType = FIVEOFAKIND
	} else if isFourOfAKind(hand) {
		handType = FOUROFAKIND
	} else if isFullHouse(hand) {
		handType = FULLHOUSE
	} else if isThreeOfAKind(hand) {
		handType = THREEOFAKIND
	} else if isTwoPair(hand) {
		handType = TWOPAIR
	} else if isOnePair(hand) {
		handType = ONEPAIR
	}

	return handType
}

func isFiveOfAKind(hand string) bool {
	var isFiveOfAKind bool = false

	var firstChar string = string(hand[0])

	if strings.Count(hand, firstChar) == 5 {
		isFiveOfAKind = true
	}

	return isFiveOfAKind
}

func isFourOfAKind(hand string) bool {
	var isFourOfAKind bool = false

	var firstChar string = string(hand[0])
	var secondChar string = string(hand[1])

	if strings.Count(hand, firstChar) == 4 || strings.Count(hand, secondChar) == 4 {
		isFourOfAKind = true
	}

	return isFourOfAKind
}

func isFullHouse(hand string) bool {
	var isFullHouse bool = false

	// Check for a full house (three of a kind and a pair)
	var uniqueChars = make(map[string]int)

	for _, card := range hand {
		uniqueChars[string(card)]++
	}

	var hasThreeOfAKind bool
	var hasPair bool

	for _, count := range uniqueChars {
		if count == 3 {
			hasThreeOfAKind = true
		} else if count == 2 {
			hasPair = true
		}
	}

	if hasThreeOfAKind && hasPair {
		isFullHouse = true
	}

	return isFullHouse
}

func isThreeOfAKind(hand string) bool {
	// Check for three of a kind
	var uniqueChars = make(map[string]int)

	for _, card := range hand {
		uniqueChars[string(card)]++
	}

	for _, count := range uniqueChars {
		if count == 3 {
			return true
		}
	}

	return false
}

func isTwoPair(hand string) bool {
	// Check for two pairs
	var uniqueChars = make(map[string]int)
	var pairCount int

	for _, card := range hand {
		uniqueChars[string(card)]++
		if uniqueChars[string(card)] == 2 {
			pairCount++
		}
	}

	return pairCount == 2
}

func isOnePair(hand string) bool {
	// Check for one pair
	var uniqueChars = make(map[string]int)

	for _, card := range hand {
		uniqueChars[string(card)]++
	}

	for _, count := range uniqueChars {
		if count == 2 {
			return true
		}
	}

	return false
}

func calculateTotalWinnings(bets []int) int {
	var totalWinnings int = 0

	for i := 0; i < len(bets); i++ {
		totalWinnings += bets[i] * (i + 1)
	}

	return totalWinnings
}