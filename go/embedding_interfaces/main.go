package main

import "fmt"

type Parent interface {
	Child1
	Child2
}

type Child1 interface{
	// Follow line error duplicate
	// Raw() string 
	Str() string
}

type Child2 interface{
	Raw() float64
}

type Quantity float64

func (q Quantity) Str() string {
	return fmt.Sprint(q)
}

func (q Quantity) Raw() float64 {
	return float64(q)
}

func main() {
	var i Parent 
	i = Quantity(500.15)
	fmt.Println("Str: ", i.Str(), ", Raw: ", i.Raw())
}
