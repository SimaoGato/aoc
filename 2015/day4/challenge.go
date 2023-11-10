package main

import (
	"crypto/md5"
	"encoding/hex"
	"fmt"
	"strconv"
)

func findAdventCoin(secretKey string) int {
	for i := 1; ; i++ {
		input := secretKey + strconv.Itoa(i)
		hash := md5.Sum([]byte(input))
		hashString := hex.EncodeToString(hash[:])

		// Check if the hash starts with at least five zeroes
		if hashString[:6] == "000000" {
			return i
		}
	}
}

func main() {
	secretKey := "yzbqklnj"
	result := findAdventCoin(secretKey)
	fmt.Printf("The lowest positive number to produce the required hash is: %d\n", result)
}
