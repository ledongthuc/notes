package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	tailPositions(2)
	tailPositions(10)
}

func tailPositions(numberOfKnot int) {
	insts, err := buildInstructions()
	if err != nil {
		panic(err)
	}

	w := make(walkMap)
	r := rope{
		knots: make([]point, numberOfKnot),
	}
	for _, inst := range insts {
		for step := uint(0); step < inst.moves; step++ {
			r.knots[0].move(directionMap[inst.side])
			for index := 1; index < len(r.knots)-1; index++ {
				r.knots[index].closeTo(r.knots[index-1])
			}
			r.knots[len(r.knots)-1].closeTo(r.knots[len(r.knots)-2])
			w[r.knots[len(r.knots)-1]]++
		}
	}

	fmt.Println("Number of positions the tail of the rope visit at least once: ", w.Count())
}

const (
	R = "R"
	L = "L"
	U = "U"
	D = "D"
)

type point struct {
	x int
	y int
}

var directionMap = map[string]point{
	R: {x: 1, y: 0},
	L: {x: -1, y: 0},
	U: {x: 0, y: 1},
	D: {x: 0, y: -1},
}

func (from *point) move(direction point) {
	from.x += direction.x
	from.y += direction.y
}

func (from *point) closeTo(to point) {
	diffX := to.x - from.x
	diffY := to.y - from.y
	if int(math.Abs(float64(diffX))) > 1 || int(math.Abs(float64(diffY))) > 1 {
		from.x += sign(diffX)
		from.y += sign(diffY)
	}
}

func sign(x int) int {
	if x == 0 {
		return 0
	}
	return int(math.Copysign(1, float64(x)))

}

type rope struct {
	knots []point
}

type walkMap map[point]uint

func (w walkMap) Count() int {
	return len(w)
}

type instruction struct {
	side  string
	moves uint
}

func buildInstructions() ([]instruction, error) {
	var insts []instruction
	err := readLineByLine(func(line string) error {
		parts := strings.Split(line, " ")
		move, err := strconv.ParseUint(parts[1], 10, 32)
		if err != nil {
			return err
		}
		insts = append(insts, instruction{
			side:  parts[0],
			moves: uint(move),
		})
		return nil
	})
	return insts, err
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
