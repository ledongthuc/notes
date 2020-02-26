/*
                           ┌───────┐
                           │       │
                           │       │
┌────────┐            ┌────────┐   │
│   d4   │            │   d5   │   │
└────────┘            └────────┘   │
     ▲                     ▲       │
     └──────────┬──────────┘       │        ───────┐
                │                  │   ┌────────┐  │
                │                  │   │   d6   │  │
           ┌────────┐              │   └────────┘  │
           │   d3   │              │        ▲      │
           └────────┘              │        │      │
                ▲                  └────────┤      │
     ┌──────────┴────────────┐              │      │
     │                       │              │      │
┌────────┐              ┌────────┐          │      │
│   d1   │              │   d2   │──────────┘      │
└────────┘              └────────┘                 │
     ▲                                             │
     └─────────────────────────────────────────────┘
*/
package main

import "fmt"

type d1 interface {
	d6
	Action() string
	D1()
}

type d2 interface {
	Action() string
	D2()
}

type d3 interface {
	d1
	d2
	Action() string
	D3()
}

type d4 interface {
	d3
	Action() string
	D4()
}

type d5 interface {
	d3
	Action() string
	D5()
}

type d6 interface {
	d2
	d5
	Action() string
	D6()
}

type concreate struct{}

func (c concreate) Action() string {
	s := "Action()"
	fmt.Println(s)
	return s
}

func (c concreate) D1() {
	fmt.Println("D1")
}

func (c concreate) D2() {
	fmt.Println("D2")
}

func (c concreate) D3() {
	fmt.Println("D3")
}

func (c concreate) D5() {
	fmt.Println("D5")
}

func (c concreate) D6() {
	fmt.Println("D6")
}

func main() {
	var child d6
	child = concreate{}
	child.Action()
	child.D2()
}
