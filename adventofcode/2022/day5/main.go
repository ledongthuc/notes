package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"unicode"
)

func main() {
	arrange(true)
	arrange(false)
}

func arrange(withReverseMoving bool) {
	crates, insts, err := parseFileContent()
	if err != nil {
		panic(err)
	}

	// run instructions
	for _, inst := range insts {
		vs := crates[inst.from][:inst.count:inst.count]
		if withReverseMoving {
			for index := 0; index < len(vs)/2; index++ {
				vs[index], vs[len(vs)-index-1] = vs[len(vs)-index-1], vs[index]
			}
		}

		crates[inst.from] = crates[inst.from][inst.count:]
		crates[inst.to] = append(vs, crates[inst.to]...)
	}

	for _, column := range crates {
		if len(column) > 0 {
			fmt.Print(column[0])
		}
	}
	fmt.Print("\n")
}

type instruction struct {
	count uint
	from  uint
	to    uint
}

func parseFileContent() (crates [][]string, ins []instruction, err error) {
	isCratePart := true
	err = readLineByLine(func(line string) error {
		switch {
		case isCratePart && line != "":
			line = line + " "
			for i := 0; i < len(line)/4; i++ {
				if len(crates) <= i {
					crates = append(crates, []string{})
				}

				v := strings.Trim(line[4*i:4*i+4], "[] ")
				if v != "" && !unicode.IsNumber(rune(v[0])) {
					crates[i] = append(crates[i], v)
				}
			}
		case isCratePart && line == "":
			isCratePart = false
		default:
			for _, removing := range []string{"move ", "from ", "to "} {
				line = strings.ReplaceAll(line, removing, "")
			}

			parts := strings.Split(line, " ")
			count, err := strconv.ParseUint(parts[0], 10, 32)
			if err != nil {
				return err
			}
			from, err := strconv.ParseUint(parts[1], 10, 32)
			if err != nil {
				return err
			}
			to, err := strconv.ParseUint(parts[2], 10, 32)
			if err != nil {
				return err
			}

			ins = append(ins, instruction{
				count: uint(count),
				from:  uint(from) - 1,
				to:    uint(to) - 1,
			})
		}
		return nil
	})
	return crates, ins, err
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
