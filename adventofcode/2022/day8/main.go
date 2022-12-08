package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	m := parseMap()
	fmt.Println("The number visible: ", countVisibleTree(m))
	fmt.Println("The highest scenic score: ", findHighestScenicScore(m))
}

func findHighestScenicScore(m [][]uint) uint {
	var highestScore uint
	for rowIndex, cells := range m {
		if rowIndex == 0 || rowIndex == len(m)-1 {
			// Zero score
			continue
		}
		for columnIndex := range cells {
			if columnIndex == 0 || columnIndex == len(cells)-1 {
				// Zero score
				continue
			}

			var leftCount uint
			for i := columnIndex - 1; i >= 0; i-- {
				leftCount++
				if m[rowIndex][i] >= m[rowIndex][columnIndex] {
					break
				}
			}

			var rightCount uint
			for i := columnIndex + 1; i < len(cells); i++ {
				rightCount++
				if m[rowIndex][i] >= m[rowIndex][columnIndex] {
					break
				}
			}

			var topCount uint
			for i := rowIndex - 1; i >= 0; i-- {
				topCount++
				if m[i][columnIndex] >= m[rowIndex][columnIndex] {
					break
				}
			}

			var bottomCount uint
			for i := rowIndex + 1; i < len(m); i++ {
				bottomCount++
				if m[i][columnIndex] >= m[rowIndex][columnIndex] {
					break
				}
			}

			score := leftCount * rightCount * topCount * bottomCount
			if score > highestScore {
				highestScore = score
			}
		}
	}
	return highestScore
}

func countVisibleTree(m [][]uint) uint {
	var noVisibleTree uint
	for rowIndex, cells := range m {
		if rowIndex == 0 || rowIndex == len(m)-1 {
			noVisibleTree += uint(len(cells))
			continue
		}
		for columnIndex := range cells {
			if columnIndex == 0 || columnIndex == len(cells)-1 {
				noVisibleTree++
				continue
			}

			if isMiddleVisibleTree(rowIndex, columnIndex, m) {
				noVisibleTree++
			}
		}
	}
	return noVisibleTree
}

func isMiddleVisibleTree(rowIndex, columnIndex int, m [][]uint) bool {
	visible := true
	for i := 0; i < columnIndex; i++ {
		if m[rowIndex][i] >= m[rowIndex][columnIndex] {
			visible = false
			break
		}
	}
	if visible {
		return visible
	}

	visible = true
	for i := columnIndex + 1; i < len(m[0]); i++ {
		if m[rowIndex][i] >= m[rowIndex][columnIndex] {
			visible = false
			break
		}
	}
	if visible {
		return visible
	}

	visible = true
	for i := 0; i < rowIndex; i++ {
		if m[i][columnIndex] >= m[rowIndex][columnIndex] {
			visible = false
			break
		}
	}
	if visible {
		return visible
	}

	visible = true
	for i := rowIndex + 1; i < len(m); i++ {
		if m[i][columnIndex] >= m[rowIndex][columnIndex] {
			visible = false
			break
		}
	}
	return visible
}

func parseMap() [][]uint {
	var m [][]uint
	err := readLineByLine(func(line string) error {
		row := make([]uint, 0, len(line))
		for _, r := range line {
			c, err := strconv.ParseUint(string(r), 10, 32)
			if err != nil {
				return err
			}
			row = append(row, uint(c))
		}
		m = append(m, row)
		return nil
	})
	if err != nil {
		panic(err)
	}
	return m
}

func readLineByLine(lineHandler func(line string) error) error {
	file, _ := os.Open("./input.txt")
	fscanner := bufio.NewScanner(file)
	for fscanner.Scan() {
		if err := lineHandler(fscanner.Text()); err != nil {
			return err
		}
	}
	return nil
}
