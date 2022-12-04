package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	var coverCounter uint
	var overlapCounter uint

	err := readLineByLine(func(line string) error {
		elfRanges, err := parse2ElfRanges(line)
		if err != nil {
			return err
		}
		if elfRanges[0].isCover(elfRanges[1]) || elfRanges[1].isCover(elfRanges[0]) {
			coverCounter++
		}
		if elfRanges[0].isOverlap(elfRanges[1]) || elfRanges[1].isOverlap(elfRanges[0]) {
			overlapCounter += 1
		}
		return nil
	})
	if err != nil {
		panic(err)
	}

	fmt.Println("Number of assignment pairs does one range fully contain the other: ", coverCounter)
	fmt.Println("Number of overlap: ", overlapCounter)
}

type ElfRange struct {
	from uint
	to   uint
}

func parse2ElfRanges(raw string) ([2]ElfRange, error) {
	ranges := strings.Split(raw, ",")
	if len(ranges) != 2 {
		return [2]ElfRange{}, fmt.Errorf("'%s' is not 2 pairs", raw)
	}

	var result [2]ElfRange
	for index, item := range ranges {
		fromTo := strings.Split(item, "-")
		if len(fromTo) != 2 {
			return [2]ElfRange{}, fmt.Errorf("'%s' of '%s' is not enough from-to format", item, raw)
		}

		from, err := strconv.Atoi(fromTo[0])
		if err != nil {
			return [2]ElfRange{}, fmt.Errorf("'%s' of '%s' can't parse from", item, raw)
		}
		to, err := strconv.Atoi(fromTo[1])
		if err != nil {
			return [2]ElfRange{}, fmt.Errorf("'%s' of '%s' can't parse from", item, raw)
		}

		result[index] = ElfRange{
			from: uint(from),
			to:   uint(to),
		}
	}
	return result, nil

}

func (e1 ElfRange) isCover(e2 ElfRange) bool {
	return e1.from <= e2.from && e1.to >= e2.to
}

func (e1 ElfRange) isOverlap(e2 ElfRange) bool {
	return (e1.from <= e2.from && e1.to >= e2.from) ||
		(e1.to <= e2.to && e1.to >= e2.to)
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
