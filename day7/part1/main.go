package main

import (
	"fmt"
	"os"
	"strings"
	"strconv"
	"sort"
)

func main() {
	//open file named "../input.txt" and print each line
	content, err := os.ReadFile("../input.txt")
	if err != nil {
		fmt.Println("Error reading file")
		return
	}
	handMap := make(map[string]int)
	var hands []string
	lines := strings.Split(string(content), "\n")
	for i:= range lines {
		if(lines[i] == "") {
			continue;
		}
		vals := strings.Split(lines[i], " ")
		bet, err := strconv.Atoi(vals[1])
		if err != nil {
			fmt.Println("Error converting string to int")
			return
		}
		handMap[vals[0]] = bet
		hands = append(hands, vals[0])
	}
	
	sort.Slice(hands, func(i, j int) bool {
		return hands[i] < hands[j]
	})

	fmt.Println(hands)
}	