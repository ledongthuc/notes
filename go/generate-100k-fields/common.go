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
