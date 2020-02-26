package main

import "fmt"

type d1 interface {
	d2
	Action() string
}

type d2 interface {
	d3
	Action() string
}

type d3 interface {
	d4
	Action() string
}

type d4 interface {
	d1
	Action() string
}

type concreate struct{}

func (c concreate) Action() string {
	s := "Action()"
	fmt.Println(s)
	return s
}

func main() {
	var child d1
	child = concreate{}
	child.Action()
}
