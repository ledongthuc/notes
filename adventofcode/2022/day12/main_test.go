package main

import "testing"

func Test_point_canWalkTo(t *testing.T) {
	m := [][]rune{
		{'a', 'b'},
		{'c', 'b'},
	}
	p := point{0, 0}
	actual := p.canWalkTo(point{1, 0}, m)
	if actual != true {
		t.Errorf("Expected: true, actual: %v. Current point: %c, compare point: %c", actual, m[0][0], m[1][0])
	}
}
