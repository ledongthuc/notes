package main

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strconv"
	"strings"
)

func main() {
	solve(20, 3)
	solve(10000, 1)
}

func solve(n int, dividedRound uint64) {
	monkeys := LoadMonkeys()

	commonDiv := uint64(1)
	for _, monkey := range monkeys {
		commonDiv *= monkey.TestDivide
	}

	counter := make([]int, len(monkeys))
	for i := 0; i < n; i++ {
		for mIndex, monkey := range monkeys {
			for _, item := range monkey.Items {
				newWorryLevelItem := uint64((monkey.OperationFnc(item) % commonDiv) / dividedRound) // worried
				counter[mIndex]++

				if newWorryLevelItem%monkey.TestDivide == 0 {
					monkeys[monkey.TestMoveTrue].Items = append(monkeys[monkey.TestMoveTrue].Items, newWorryLevelItem)
				} else {
					monkeys[monkey.TestMovefalse].Items = append(monkeys[monkey.TestMovefalse].Items, newWorryLevelItem)
				}
			}
			monkeys[mIndex].Items = []uint64{}
		}

	}

	sort.Ints(counter)
	fmt.Println("The level of monkey business after ", n, " rounds of stuff-slinging simian shenanigans: ", counter[len(counter)-1]*counter[len(counter)-2])
}

func LoadMonkeys() []Monkey {
	b, err := ioutil.ReadFile("./input.txt")
	if err != nil {
		panic(err)
	}

	parser := MonkeysParser{
		Buffer: string(b),
		Pretty: true,
	}

	err = parser.Init()
	if err != nil {
		panic(err)
	}

	err = parser.Parse()
	if err != nil {
		panic(err)
	}

	var monkeys []Monkey
	monkeyAST := parser.AST().up
	for ; monkeyAST != nil; monkeyAST = monkeyAST.next {
		if monkeyAST.pegRule != rulemonkey {
			continue
		}

		// ID
		id, err := strconv.ParseUint(getNodeValue(parser.Buffer, monkeyAST.up.up), 10, 32)
		if err != nil {
			panic(err)
		}

		// Items
		var startingItems []uint64
		for _, item := range strings.Split(getNodeValue(parser.Buffer, monkeyAST.up.next.next.up), ",") {
			item, err := strconv.ParseUint(strings.TrimSpace(item), 10, 32)
			if err != nil {
				panic(err)
			}
			startingItems = append(startingItems, uint64(item))
		}

		// Op
		operation := monkeyAST.up.next.next.next.next.up
		// operationLeft := getNodeValue(parser.Buffer, operation.up)
		operationOperand := getNodeValue(parser.Buffer, operation.up.next)
		operationRight := getNodeValue(parser.Buffer, operation.up.next.next)
		opFunc := func(u uint64) uint64 {
			left := u
			right := u
			if operationRight != "old" {
				v, err := strconv.ParseUint(strings.TrimSpace(operationRight), 10, 32)
				if err != nil {
					panic(err)
				}
				right = uint64(v)
			}
			switch operationOperand {
			case "+":
				return left + right
			case "*":
				return left * right
			}
			return u
		}

		monkeyDivide, err := strconv.ParseUint(getNodeValue(parser.Buffer, monkeyAST.up.next.next.next.next.next.next.up), 10, 32)
		if err != nil {
			panic(err)
		}

		monkeyIDTrue, err := strconv.ParseUint(getNodeValue(parser.Buffer, monkeyAST.up.next.next.next.next.next.next.up.next.next.up), 10, 32)
		if err != nil {
			panic(err)
		}

		monkeyIDFalse, err := strconv.ParseUint(getNodeValue(parser.Buffer, monkeyAST.up.next.next.next.next.next.next.up.next.next.next.next.up), 10, 32)
		if err != nil {
			panic(err)
		}

		monkeys = append(monkeys, Monkey{
			ID:               uint64(id),
			Items:            startingItems,
			OperationFnc:     opFunc,
			OperationOperand: operationOperand,
			OperationRight:   operationRight,
			TestDivide:       uint64(monkeyDivide),
			TestMoveTrue:     uint64(monkeyIDTrue),
			TestMovefalse:    uint64(monkeyIDFalse),
		})
	}

	return monkeys
}

func getNodeValue(buffer string, node *node32) string {
	return string(([]rune(buffer)[node.begin:node.end]))
}

type Monkey struct {
	ID               uint64              `json:"id"`
	Items            []uint64            `json:"items"`
	OperationFnc     func(uint64) uint64 `json:"-"`
	OperationOperand string
	OperationRight   string
	TestDivide       uint64
	TestMoveTrue     uint64
	TestMovefalse    uint64
}
