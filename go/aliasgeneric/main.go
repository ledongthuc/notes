package main

import (
	"fmt"
	"reflect"
)

func main() {
	thing := Thing[int]{
		Part: 1,
	}
	CheckThing(thing)

	thingAlias := ThingAlias[string]{
		Part: "1",
	}
	CheckThing(thingAlias)

}

type ThingAlias[T any] = Thing[T]

type Thing[T any] struct {
	Part T
}

func CheckThing[T any](thing Thing[T]) {
	fmt.Println("DEBUG: type of thing: ", reflect.TypeOf(thing).Name())
}
