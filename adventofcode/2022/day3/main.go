package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	part1()
	part2()
}

func part1() {
	sumPriorityAppearedBothCompartments := 0

	readLineByLine(func(line string) error {
		if len(line) == 0 {
			return nil
		}

		if len(line)%2 != 0 {
			return fmt.Errorf("can't split same things to two compartments: %s", line)
		}

		var left [53]bool
		var right [53]bool
		for i := 0; i < len(line)/2; i++ {
			left[getPriority([]rune(line)[i])] = true
			right[getPriority([]rune(line)[i+len(line)/2])] = true
		}

		for index := range left {
			if left[index] && right[index] {
				sumPriorityAppearedBothCompartments += index
			}
		}

		return nil
	})

	fmt.Println("The sum of the priorities of both compartments: ", sumPriorityAppearedBothCompartments)
}

func part2() {
	sum := 0

	err := read3Lines(func(lines [3]string) error {
		var snackCounter [3][53]bool

		for snackIndex, items := range lines {
			for _, c := range items {
				snackCounter[snackIndex][getPriority(c)] = true
			}
		}

		for index := range snackCounter[0] {
			if snackCounter[0][index] && snackCounter[1][index] && snackCounter[2][index] {
				sum += index
			}
		}

		return nil
	})
	if err != nil {
		panic(err)
	}

	fmt.Println("The sum of group's badge: ", sum)
}

func getPriority(c rune) int {
	if c >= 'a' {
		return int(c) - 'a' + 1
	} else {
		return int(c) - 'A' + 1 + 26
	}
}

func read3Lines(linesHandler func(lines [3]string) error) error {
	index := 0
	var buffer [3]string
	return readLineByLine(func(line string) error {
		buffer[index] = line
		index++
		if index == 3 {
			if err := linesHandler(buffer); err != nil {
				return err
			}
			index = 0
		}
		return nil
	})
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
