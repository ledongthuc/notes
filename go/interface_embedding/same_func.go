package main

import "fmt"

type parent interface {
	Action()
	ParentFunction()
}

type child interface {
	parent
	Action()
	ChildFunction()
}

type concreate struct{}

func (c concreate) Action() {
	fmt.Println("Action()")
}

func (c concreate) ChildFunction() {
	fmt.Println("ChildFunction()")
}

func (c concreate) ParentFunction() {
	fmt.Println("ParentFunction()")
}

func main() {
	var c child
	c = concreate{}
	c.ChildFunction()
	c.ParentFunction()
	c.Action()
}
