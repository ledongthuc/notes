package main

import "fmt"

type parent interface {
	ParentFunction()
}

type child interface {
	parent
	ChildFunction()
}

type concreate struct{}

func (c concreate) ChildFunction() {
	fmt.Println("ChildFunction()")
}

func (c concreate) ParentFunction() {
	fmt.Println("ParentFunction()")
}

func main() {
	child := concreate{}
	child.ChildFunction()
	child.ParentFunction()
}
