package main

import (
	"bufio"
	"fmt"
	"os"

	"github.com/ledongthuc/goterators"
)

func main() {
	m := load()
	shortestPath := walk(m.start, m, []point{}, 0)
	fmt.Println("DEBUG: ", shortestPath-2)
}

func walk(myPosition point, m theMap, myPath []point, count int) int {
	if myPosition.isSame(m.end) {
		return count
	}
	nextPoints := findNextStep(myPosition, m.width, m.height, myPath, m.data)
	smallestCount := -1
	for _, nextPoint := range nextPoints {
		count := walk(nextPoint, m, append(myPath, nextPoint), count+1)
		if count == -1 {
			continue
		}

		if smallestCount == -1 || count < smallestCount {
			smallestCount = count
		}
	}
	return smallestCount
}

func printMap(path []point, m theMap) {
	for rowIndex, row := range m.data {
		for colIndex := range row {
			if goterators.Exist(path, point{rowIndex, colIndex}) {
				fmt.Print("X")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}
}

type point struct {
	x, y int
}

func (p1 point) isSame(p2 point) bool {
	return p1.x == p2.x && p1.y == p2.y
}

func (p1 point) canWalkTo(p2 point, data [][]rune) bool {
	return data[p2.y][p2.x] <= data[p1.y][p1.x]+1
}

func (p point) validPoint(mapWidth, mapHeight int) bool {
	return p.x >= 0 && p.x < mapWidth && p.y >= 0 && p.y < mapHeight
}

func findNextStep(p point, width, height int, noDuplicates []point, data [][]rune) []point {
	result := []point{
		{x: p.x - 1, y: p.y},
		{x: p.x, y: p.y - 1},
		{x: p.x + 1, y: p.y},
		{x: p.x, y: p.y + 1},
	}
	result = goterators.Filter(result, func(item point) bool {
		return item.validPoint(width, height) && p.canWalkTo(item, data) && !goterators.Exist(noDuplicates, item)
	})
	return result
}

type theMap struct {
	start  point
	end    point
	width  int
	height int
	data   [][]rune
}

func (m theMap) generatePointMap() [][]int {
	pointMap := make([][]int, m.height)
	for index := range pointMap {
		pointMap[index] = make([]int, m.width)
		for cellIndex := range pointMap[index] {
			if m.start.x == cellIndex && m.start.y == index {
				pointMap[index][cellIndex] = 0
			} else {
				pointMap[index][cellIndex] = -1
			}
		}
	}
	return pointMap
}

func load() theMap {
	var m theMap
	err := readLineByLine(func(line string) error {
		m.width = len(line)
		row := make([]rune, len(line))
		for index, c := range line {
			switch c {
			case 'S':
				m.start = point{x: index, y: m.height}
				row[index] = 'a'
			case 'E':
				m.end = point{x: index, y: m.height}
				row[index] = 'z'
			default:
				row[index] = c
			}
		}
		m.height++
		m.data = append(m.data, row)
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
