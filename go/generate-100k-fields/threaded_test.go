package test

import (
	"math/rand"
	"runtime"
	"testing"
	"time"
)

func BenchmarkThreaded(b *testing.B) {
	threaded()
}

func threaded() {
	noThread := runtime.NumCPU()
	fieldChan := make(chan field, total/noThread)

	for i := 0; i < noThread; i++ {
		go func(i int, collector chan<- field) {
			r := rand.New(rand.NewSource(time.Now().UTC().UnixNano() + int64(i)))
			for j := 0; j < total/noThread; j++ {
				collector <- generateField(r)
			}
		}(i, fieldChan)
	}

	groupChan := make(chan fields, total/minBatchSize)
	go fieldGrouping(fieldChan, groupChan)

	dbHandler(groupChan)
}
