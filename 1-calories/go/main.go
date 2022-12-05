package main

import (
	"fmt"
	"os"
	"strings"
	"strconv"
)

func max(s []int) (max int, index int) {
	max = s[0]
	index = 0

	for i, value := range s {
		if value > max {
			max = value
			index = i
		}
	}

	return
}

func remove(s []int, i int) []int {
    s[i] = s[len(s)-1]
    return s[:len(s)-1]
}

func sum(s []int) (sum int) {
	for _, v := range s {
		sum += v
	}
	return
}

func main() {
	file, err := os.ReadFile("../calories.txt")

	if err != nil {
		panic(err)
	}

	fileContent := string(file)

	caloriesList := strings.Split(fileContent, "\n")

	calories := []int{0}
	
	person := 0

	for _, val := range caloriesList {
		if val == "" {
			calories = append(calories, 0)
			person += 1
			continue
		}

		numCalories, err := strconv.Atoi(val)

		if (err != nil) {
			panic(err)
		}

		calories[person] += numCalories
	}

	topCalories := []int{}

	for i := 0; i < 3; i++ {
		maxCalorie, index := max(calories)
		topCalories = append(topCalories, maxCalorie)
		calories = remove(calories, index)
	}

	fmt.Println(topCalories)
	fmt.Println(sum(topCalories))
}
