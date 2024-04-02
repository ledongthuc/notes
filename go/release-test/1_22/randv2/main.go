package main

import (
	"fmt"
	"math/rand/v2"
	"time"
)

var predefined = []string{
	"James",
	"Oliver",
	"Benjamin",
	"Elijah",
	"Henry",
	"Jack",
	"Mateo",
	"Noah",
	"William",
}

func main() {
	fmt.Println("pick a name: ", predefined[rand.IntN(len(predefined))])

	r := rand.New(rand.NewPCG(1, 2))
	fmt.Println("random with seed. Float32: ", r.Float32())
	fmt.Println("random with seed. Float64: ", r.Float64())
	fmt.Println("random with seed. ExpFloat64: ", r.ExpFloat64())
	fmt.Println("random with seed. N: ", rand.N(100*time.Second))
	fmt.Println("random with seed. Perm: ", rand.Perm(3))
	rand.Shuffle(len(predefined), func(i, j int) {
		predefined[i], predefined[j] = predefined[j], predefined[i]
	})
	fmt.Println("random with seed. After suffle: ", predefined)

}
