package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type Instruction struct {
	Type  string
	Value int
}

type Instructions []Instruction

func main() {
	content, err := ParseInput("./instruction.txt")
	if err != nil {
		panic(err)
	}
	instructions := ParseInstructions(content)
	_, _, runInstructions := RunAndCheckLoopIntruction(instructions)
	for index := range runInstructions {
		if runInstructions[index] == false {
			continue
		}
		clonedInstructions := make(Instructions, len(instructions))
		copy(clonedInstructions, instructions)
		switch clonedInstructions[index].Type {
		case "nop":
			clonedInstructions[index].Type = "jmp"
		case "jmp":
			clonedInstructions[index].Type = "nop"
		}

		value, infinity, _ := RunAndCheckLoopIntruction(clonedInstructions)
		if !infinity {
			fmt.Println("accumulator after terminated: ", value)
			break
		}
	}
}

func ParseInput(filePath string) (string, error) {
	content, err := ioutil.ReadFile(filePath)
	if err != nil {
		return "", err
	}
	return string(content), nil
}

func ParseInstructions(text string) Instructions {
	rawInstructions := strings.Split(strings.TrimSpace(text), "\n")
	instructions := Instructions{}
	for _, rawInstruction := range rawInstructions {
		parts := strings.Split(rawInstruction, " ")
		if len(parts) < 2 {
			continue
		}
		t := parts[0]
		value, err := strconv.Atoi(parts[1])
		if err != nil {
			continue
		}
		instructions = append(instructions, Instruction{
			Type:  t,
			Value: value,
		})
	}
	return instructions
}

func RunAndCheckLoopIntruction(instructions Instructions) (accumulatedValue int, isInfinity bool, runInstructions []bool) {
	if len(instructions) == 0 {
		return 0, false, []bool{}
	}
	checkedInstructions := make([]bool, len(instructions))
	runningInstructionIndex := 0
	value := 0
	for runningInstructionIndex >= 0 && runningInstructionIndex < len(instructions) && checkedInstructions[runningInstructionIndex] == false {
		checkedInstructions[runningInstructionIndex] = true
		instruction := instructions[runningInstructionIndex]
		switch instruction.Type {
		case "nop":
			runningInstructionIndex++
		case "acc":
			value += instruction.Value
			runningInstructionIndex++
		case "jmp":
			runningInstructionIndex += instruction.Value
		}
	}
	isInfinity = runningInstructionIndex < len(instructions) && checkedInstructions[runningInstructionIndex] == true
	return value, isInfinity, checkedInstructions
}
