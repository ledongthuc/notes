package test

import (
	"math/rand"
	"testing"
	"time"
)

func BenchmarkChannel(b *testing.B) {
	channel()
}

func channel() {
	fieldChan := make(chan field, minBatchSize)
	go generateFieldForChannels(total, fieldChan)

	groupChan := make(chan fields, total/minBatchSize)
	go fieldGrouping(fieldChan, groupChan)

	dbHandler(groupChan)
}

func generateFieldForChannels(count int, collector chan<- field) {
	r := rand.New(rand.NewSource(time.Now().UTC().UnixNano()))
	for i := 0; i < count; i++ {
		collector <- generateField(r)
	}
}
