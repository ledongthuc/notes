package main

import (
	"fmt"
	"runtime"
	"strings"
	"weak"
)

func main() {
	wp := PrintSomething()
	fmt.Println("DEBUG: [Before GC] wp.Value.String:", wp.Value().String())
	runtime.GC()

	if wp.Value() == nil {
		fmt.Println("DEBUG: [After GC] wp.value is released")
	} else {
		fmt.Println("DEBUG: [After GC] wp.Value.String:", wp.Value().String())
	}

}

func PrintSomething() weak.Pointer[strings.Builder] {
	var s strings.Builder
	s.WriteString("a")
	s.WriteString("b")
	s.WriteString("c")
	fmt.Println("DEBUG: s:", s.String())
	wp := weak.Make(&s)
	p := wp.Value()
	if p != nil {
		fmt.Println("DEBUG: wp.Value.String:", p.String())
	}
	return wp
}
