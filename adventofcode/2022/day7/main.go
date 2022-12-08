package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"

	"github.com/ledongthuc/goterators"
)

func main() {
	root := loadTree()
	calculateDirSized(root)
	dirSizes := getDirSizes(root)

	part1(dirSizes)
	part2(root.size[0], dirSizes)
}

func part1(dirSizes []uint64) {
	const limit = uint64(100000)
	dirSizes = goterators.Filter(dirSizes, func(item uint64) bool { return item < limit })
	fmt.Println("The sum of the total at most 100000 size: ", goterators.Sum(dirSizes))
}

func part2(rootSize uint64, dirSizes []uint64) {

	const totalDisk = uint64(70000000)
	const needFreespace = uint64(30000000)
	needToClean := needFreespace - (totalDisk - rootSize)
	fmt.Println("DEBUG: ", dirSizes[0])

	dirSizes = goterators.Filter(dirSizes, func(item uint64) bool { return item >= needToClean })
	sort.Slice(dirSizes, func(i, j int) bool { return dirSizes[i] < dirSizes[j] })
	fmt.Println("the smallest directory that, if deleted, would free up enough space on the filesystem to run the update: ", dirSizes[0])
}

func loadTree() *TreeNode {
	root := TreeNode{name: "/", isDir: true}
	var checkingPoint *TreeNode

	err := readLineByLine(func(line string) error {
		switch {
		case strings.HasPrefix(line, "$ cd"):
			pathMove := strings.TrimPrefix(line, "$ cd ")
			switch pathMove {
			case "/":
				checkingPoint = &root
			case "..":
				checkingPoint = checkingPoint.parent
			default:
				checkingPoint = FindTreeNodeByName(checkingPoint.children, pathMove)
			}
		case strings.HasPrefix(line, "$ ls"):
		case strings.HasPrefix(line, "dir "):
			name := strings.TrimPrefix(line, "dir ")
			checkingPoint.children = append(checkingPoint.children, &TreeNode{
				name: name, isDir: true, parent: checkingPoint,
			})
		default:
			parts := strings.Split(line, " ")
			size, err := strconv.ParseUint(parts[0], 10, 32)
			if err != nil {
				return err
			}
			checkingPoint.children = append(checkingPoint.children, &TreeNode{
				name: parts[1], size: Some(size), parent: checkingPoint,
			})
		}
		return nil
	})
	if err != nil {
		panic(err)
	}
	return &root
}

func calculateDirSized(node *TreeNode) uint64 {
	var total uint64
	for _, item := range node.children {
		if item.isDir {
			total += calculateDirSized(item)
			continue
		}
		total += item.size[0]
	}
	node.size = Some(total)
	return total
}

func getDirSizes(node *TreeNode) []uint64 {
	if !node.isDir {
		return []uint64{}
	}

	result := make([]uint64, 1, len(node.children)+1)
	result[0] = node.size[0]
	for _, item := range node.children {
		if !item.isDir {
			continue
		}
		result = append(result, getDirSizes(item)...)
	}

	return result
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

type TreeNode struct {
	name     string
	size     Optional[uint64]
	isDir    bool
	parent   *TreeNode
	children []*TreeNode
}

func FindTreeNodeByName(nodes []*TreeNode, name string) *TreeNode {
	for _, node := range nodes {
		if node.name == name {
			return node
		}
	}
	return nil
}

type Optional[T any] [1]T

func Some[T any](v T) Optional[T] {
	return Optional[T]{v}
}

func (o Optional[T]) Some() T {
	return o[0]
}

func (o Optional[T]) IsNone() bool {
	return len(o) == 0
}
