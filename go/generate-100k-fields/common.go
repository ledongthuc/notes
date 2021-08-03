package test

import (
	"math/rand"
	"strconv"
)

const (
	total        = 100_000_000
	minBatchSize = 1_000_000
)

var (
	batchCount = total / minBatchSize
)

type field struct {
	hasArea bool
	area    string
	active  int
	age     int
}

type fields []field

var (
	availableAges = []int{
		5, 10, 15,
	}
)

func getRandomBool(r *rand.Rand) bool {
	return r.Intn(2) == 1
}

func getRandomActive(r *rand.Rand) int {
	return r.Intn(2)
}

func getRandomAge(r *rand.Rand) int {
	return availableAges[r.Intn(3)]
}

func getRandomArea(r *rand.Rand) string {
	return strconv.FormatInt(int64(r.Intn(999999-100000+1)+100000), 10)
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
