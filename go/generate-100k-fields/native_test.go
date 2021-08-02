package test

import (
	"math/rand"
	"testing"
	"time"
)

func BenchmarkNative(b *testing.B) {
	native()
}

func native() {
	r := rand.New(rand.NewSource(time.Now().UTC().UnixNano()))
	for i := 1; i < total/minBatchSize; i++ {
		hasArea := getRandomBool(r)
		currentBatch := []interface{}{}
		for i := 0; i < minBatchSize; i++ {
			age := getRandomAge(r)
			active := getRandomActive(r)
			area := getRandomArea(r)
			if hasArea {
				currentBatch = append(currentBatch, []interface{}{
					area, age, active,
				})
			} else {
				currentBatch = append(currentBatch, []interface{}{
					age, active,
				})
			}
		}
	}
}
