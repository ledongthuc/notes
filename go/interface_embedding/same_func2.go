package main

import "fmt"

type parent interface {
	Action() string
	ParentFunction()
}

type child interface {
	parent
	Action() int
	ChildFunction()
}

type concreate struct{}

func (c concreate) Action() string {
	s := "Action()"
	fmt.Println(s)
	return s
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
