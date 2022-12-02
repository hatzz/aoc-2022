package main

import (
	"fmt"
	"os"
	"strings"
)

type GameOutcome int
type Shape int

const (
	Rock     Shape = 1
	Paper          = 2
	Scissors       = 3
)

const (
	Win  GameOutcome = 6
	Draw             = 3
	Loss             = 0
)

type Game struct {
	player   Shape
	opponent Shape
}

func getOpponentShape(s string) Shape {
	switch s {
	case "A":
		return Rock
	case "B":
		return Paper
	case "C":
		return Scissors
	default:
		panic("Invalid shape!")
	}
}

func getPlayerShape(s string) Shape {
	switch s {
	case "X":
		return Rock
	case "Y":
		return Paper
	case "Z":
		return Scissors
	default:
		panic("Invalid shape!")
	}
}

// Part 2
func getGameOutcome(g string) GameOutcome {
	switch g {
	case "X":
		return Loss
	case "Y":
		return Draw
	case "Z":
		return Win
	default:
		panic("Invalid game outcome!")
	}
}

func getOpposingShape(s Shape) Shape {
	switch s {
	case Rock:
		return Paper
	case Paper:
		return Scissors
	case Scissors:
		return Rock
	default:
		panic("Invalid shape")
	}
}

func getPlayerShapeFromOutcome(opponent Shape, g GameOutcome) Shape {
	switch g {
	case Win:
		return getOpposingShape(opponent)
	case Draw:
		return opponent
	case Loss:
		return getOpposingShape(getOpposingShape(opponent))
	default:
		panic("Invalid game outcome")
	}
}

func playGame(g Game) GameOutcome {
	switch {
	case g.player == g.opponent:
		return Draw
	case g.player == Rock && g.opponent == Scissors:
		return Win
	case g.player == Paper && g.opponent == Rock:
		return Win
	case g.player == Scissors && g.opponent == Paper:
		return Win
	default:
		return Loss
	}
}

func getGame(player, opponent string) Game {
	return Game{
		player:   getPlayerShape(player),
		opponent: getOpponentShape(opponent),
	}
}

func splitGameString(g string) (string, string) {
	gameArray := strings.Split(g, " ")

	// This is scary, what if these are nil?
	// Apparently it pancis!
	return gameArray[0], gameArray[1]
}

func playGames(games []Game) int {
	playerScore := 0

	for _, game := range games {
		playerScore += int(game.player)
		gameOutcome := playGame(game)
		playerScore += int(gameOutcome)
	}

	return playerScore
}

func getGamesPart1(gameStrings []string) []Game {
	games := []Game{}

	for _, gameString := range gameStrings {
		if gameString == "" {
			continue
		}
		opponent, player := splitGameString(gameString)
		games = append(games, getGame(player, opponent))
	}

	return games
}

func getGamesPart2(gameStrings []string) []Game {
	games := []Game{}

	for _, gameString := range gameStrings {
		if gameString == "" {
			continue
		}
		opponent, player := splitGameString(gameString)

		opponentShape := getOpponentShape(opponent)
		gameOutcome := getGameOutcome(player)
		playerShape := getPlayerShapeFromOutcome(opponentShape, gameOutcome)

		games = append(games, Game{player: playerShape, opponent: opponentShape})
	}

	return games
}

func main() {
	file, err := os.ReadFile("rock-paper-scissors.txt")

	if err != nil {
		panic(err)
	}

	gameStrings := strings.Split(string(file), "\n")
	part1Games := getGamesPart1(gameStrings)
	part2Games := getGamesPart2(gameStrings)

	fmt.Printf("Part 1: %d\n", playGames(part1Games))
	fmt.Printf("Part 2: %d\n", playGames(part2Games))
}
