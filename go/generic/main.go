package main

import "fmt"

// Implement of Stringer
type Item struct {
	Name     string
	Quantity int
}

func (i Item) String() string {
	return fmt.Sprintf("%s: %d", i.Name, i.Quantity)
}

type Stringer interface {
	String() string
}

type Number interface {
	Raw() float64
}

// Generic interface
func Stringfy[T Stringer](s []T) (ret []string) {
	for _, v := range s {
		ret = append(ret, v.String())
	}
	return ret
}

// Generic reference it self
type LinkedItem[T1 any, T2 any] struct {
	ChildItems []LinkedItem[T1, T2]
}

// Type Generic
type Vector[T any] []T

func (v *Vector[T]) Push(x T) { *v = append(*v, x) }

// Underline with string type
type AnyString interface {
	~string
}

type MyString string

func PrintAnyString[T AnyString](s T) {
	fmt.Println(s)
}

// Union contrants types
type UnionSignedNumber interface {
	~uint | ~uint8 | ~uint16 | ~uint32 | ~uint64
}

func SumUnionSignedNumber[T UnionSignedNumber](x, y T) T {
	return x + y
}

func PrintMaxUnionSignedNumber[T UnionSignedNumber](x, y T) {
	if x > y {
		fmt.Printf("%v > %v\n", x, y)
	}
	if x < y {
		fmt.Printf("%v < %v\n", x, y)
	}
	if x == y {
		fmt.Printf("%v == %v\n", x, y)
	}
}

// Collection funcs

func Filter[T any](source []T, filterFunc func(item T) bool) (out []T) {
	for _, item := range source {
		if filterFunc(item) {
			out = append(out, item)
		}
	}
	return out
}

func Map[T1 any, T2 any](source []T1, mapFunc func(item T1) T2) (out []T2) {
	for _, item := range source {
		out = append(out, mapFunc(item))
	}
	return out
}

func Find[T any](source []T, conditionFunc func(item T) bool) (out T, err error) {
	for _, item := range source {
		if conditionFunc(item) {
			return item, nil
		}
	}
	return out, fmt.Errorf("does not exist")
}

func main() {
	fmt.Println("Print Implicit:")
	Print[int]([]int{1, 2, 3, 4})

	fmt.Println("Print Explicit:")
	Print([]int{1, 2, 3, 4})

	fmt.Println("Stringfy: ", Stringfy([]Item{
		{Name: "Test 1", Quantity: 1},
		{Name: "Test 2", Quantity: 2},
		{Name: "Test 3", Quantity: 3},
	}))

	intVector := Vector[int]{1, 2, 3}
	intVector.Push(4)
	fmt.Println("intVector: ", intVector)

	floatVector := Vector[float64]{1.1, 2.2, 3.3, 4.4}
	floatVector.Push(4.4)
	fmt.Println("floatVector: ", floatVector)

	// Type Set example
	// itemTypeSet := ItemTypeSet{Name: "Test 1",Quantity: 1}
	// PrintTypeSet[ItemTypeSet](itemTypeSet)

	// Test filter by define input function
	sourceFilter := []int{1, 2, 3, 4, 5, 6, 7, 8}
	outFilter := Filter(sourceFilter, func(item int) bool {
		return item%2 == 0
	})
	fmt.Println("OutFilter: ", outFilter)

	// Test mapping
	sourceMap := []int{1, 2, 3, 4, 5, 6, 7, 8}
	outMap := Map(sourceMap, func(item int) string {
		return fmt.Sprintf("%d-item", item)
	})
	fmt.Println("OutMap: ", outMap)

	// Test find
	sourceFind := []int{1, 2, 3, 4, 5, 6, 7, 8}
	matchedItem, err := Find(sourceFind, func(item int) bool {
		return item == 4
	})
	fmt.Println("Find: ", matchedItem, err)

	matchedItem, err = Find(sourceFind, func(item int) bool {
		return item == 12
	})
	fmt.Println("Find: ", matchedItem, err)

	// Use unterlying type of string
	PrintAnyString("Print AnyString: string")
	PrintAnyString(MyString("Print AnyString: MyString"))

	// Use union contraint types
	fmt.Println("SumUnionSignedNumber :", SumUnionSignedNumber(uint(1), uint(2)))
	// Follow line failed
	// fmt.Println("SumUnionSignedNumber :", SumUnionSignedNumber(uint(1), uint(-2)))
	PrintMaxUnionSignedNumber(uint(1), uint(2))
}

func Print[T any](s []T) {
	for _, v := range s {
		fmt.Println(v)
	}
}
