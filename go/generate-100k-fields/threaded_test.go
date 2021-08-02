package test

import (
	"math/rand"
	"testing"
	"time"
)

func BenchmarkThreaded(b *testing.B) {
	threaded()
}

func threaded() {
	fieldChan := make(chan field, minBatchSize)
	go generateFields(total, fieldChan)

	groupChan := make(chan fields, total/minBatchSize)
	go fieldGrouping(fieldChan, groupChan)

	dbHandler(groupChan)
}

func generateFields(count int, collector chan<- field) {
	r := rand.New(rand.NewSource(time.Now().UTC().UnixNano()))
	for i := 0; i < count; i++ {
		collector <- generateField(r)
	}
}

func generateField(r *rand.Rand) field {
	age := getRandomAge(r)
	active := getRandomActive(r)
	var area string
	hasArea := getRandomBool(r)
	if hasArea {
		area = getRandomArea(r)
	}
	return field{
		hasArea: hasArea,
		area:    area,
		active:  active,
		age:     age,
	}
}

func fieldGrouping(fieldChan <-chan field, groupChan chan<- fields) {
	batch := make(fields, 0, minBatchSize)
	for i := 0; i < total; i++ {
		select {
		case f := <-fieldChan:
			batch = append(batch, f)
			if len(batch) == cap(batch) {
				groupChan <- batch
				batch = make([]field, 0, minBatchSize)
			}
		}
	}
}

func dbHandler(groupChan <-chan fields) {
	for i := 0; i < batchCount; i++ {
		select {
		case _ = <-groupChan:
		}
	}
}
