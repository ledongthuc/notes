package main

import "testing"

func TestGetPriority(t *testing.T) {
	tests := []struct {
		input    rune
		expected int
	}{
		{input: 'a', expected: 1},
		{input: 'z', expected: 26},
		{input: 'A', expected: 27},
		{input: 'Z', expected: 52},
		{input: 'p', expected: 16},
		{input: 'L', expected: 38},
		{input: 'P', expected: 42},
		{input: 'v', expected: 22},
		{input: 't', expected: 20},
	}

	for _, test := range tests {
		if v := getPriority(test.input); v != test.expected {
			t.Errorf("'%c' is %d, actual: %d", test.input, test.expected, v)
		}
	}
}
