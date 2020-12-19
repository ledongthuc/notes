package main

import (
	"fmt"
	"io/ioutil"
	"regexp"
	"strconv"
	"strings"
)

type Node struct {
	Name     string
	Children []NodeAmount
	Parents  []string
}

type NodeAmount struct {
	Name   string
	Amount int
}

type Nodes map[string]*Node

func main() {
	content, err := ParseInput("./rules.txt")
	if err != nil {
		panic(err)
	}
	nodes, err := ParseNodes(content)
	if err != nil {
		panic(err)
	}
	fmt.Println(CountOf(nodes, "shiny gold"))
}

func CountOf(nodes Nodes, name string) int {
	var counter int
	node := nodes[name]
	for _, child := range node.Children {
		counter += child.Amount + child.Amount*CountOf(nodes, child.Name)
	}

	return counter
}

func ParseNodes(rules string) (Nodes, error) {
	rawRules := strings.Split(strings.TrimSpace(rules), ".\n")
	parsedRules := Nodes{}
	for _, rule := range rawRules {
		nodeParts := strings.Split(rule, " bags contain ")
		if len(nodeParts) != 2 {
			return Nodes{}, fmt.Errorf("Rule '%s' has more than 1 part", rule)
		}
		children, err := ParseNodeAmounts(nodeParts[1])
		if err != nil {
			return Nodes{}, err
		}
		node, valid := parsedRules[nodeParts[0]]
		if !valid {
			node = &Node{}
		}
		node.Name = nodeParts[0]
		node.Children = children
		parsedRules[node.Name] = node

		for _, child := range children {
			childNode, childNodeValid := parsedRules[child.Name]
			if !childNodeValid {
				childNode = &Node{}
			}
			childNode.Name = child.Name
			childNode.Parents = append(childNode.Parents, node.Name)
			parsedRules[child.Name] = childNode
		}
	}
	return parsedRules, nil
}

func ParseNodeAmounts(rawNodeAmounts string) ([]NodeAmount, error) {
	nodeAmounts := []NodeAmount{}
	r := regexp.MustCompile(`(?P<number>\d+) (?P<name>[a-zA-Z ]+) bag(s?)`)
	matches := r.FindAllStringSubmatch(rawNodeAmounts, -1)
	for _, match := range matches {
		amount, err := strconv.Atoi(match[r.SubexpIndex("number")])
		if err != nil {
			return []NodeAmount{}, err
		}
		nodeAmounts = append(nodeAmounts, NodeAmount{
			Name:   match[r.SubexpIndex("name")],
			Amount: amount,
		})
	}
	return nodeAmounts, nil
}

func ParseInput(filePath string) (string, error) {
	content, err := ioutil.ReadFile(filePath)
	if err != nil {
		return "", err
	}
	return string(content), nil
}
