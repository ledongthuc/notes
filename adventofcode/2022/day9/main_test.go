package main

import "testing"

func Test_point_closeTo(t *testing.T) {
	from := point{x: 0, y: 0}

	// No move
	expected := point{x: 0, y: 0}
	from.closeTo(point{x: 0, y: 0})
	if from != expected {
		t.Errorf("Expected: %+v, actual: %+v", expected, from)
	}

	// Close it
	from = point{x: 0, y: 0}
	expected = point{x: 0, y: 0}
	from.closeTo(point{x: 1, y: 0})
	if from != expected {
		t.Errorf("Expected: %+v, actual: %+v", expected, from)
	}

	from = point{x: 0, y: 0}
	expected = point{x: 0, y: 0}
	from.closeTo(point{x: 0, y: 1})
	if from != expected {
		t.Errorf("Expected: %+v, actual: %+v", expected, from)
	}

	from = point{x: 0, y: 0}
	expected = point{x: 0, y: 0}
	from.closeTo(point{x: 1, y: 1})
	if from != expected {
		t.Errorf("Expected: %+v, actual: %+v", expected, from)
	}

	from = point{x: 0, y: 0}
	expected = point{x: 0, y: 0}
	from.closeTo(point{x: -1, y: 0})
	if from != expected {
		t.Errorf("Expected: %+v, actual: %+v", expected, from)
	}

	from = point{x: 0, y: 0}
	expected = point{x: 0, y: 0}
	from.closeTo(point{x: 0, y: -1})
	if from != expected {
		t.Errorf("Expected: %+v, actual: %+v", expected, from)
	}

	from = point{x: 0, y: 0}
	expected = point{x: 0, y: 0}
	from.closeTo(point{x: -1, y: -1})
	if from != expected {
		t.Errorf("Expected: %+v, actual: %+v", expected, from)
	}

	// Same row
	from = point{x: 0, y: 0}
	expected = point{x: 1, y: 0}
	from.closeTo(point{x: 2, y: 0})
	if from != expected {
		t.Errorf("Expected: %+v, actual: %+v", expected, from)
	}

	from = point{x: 0, y: 0}
	expected = point{x: -1, y: 0}
	from.closeTo(point{x: -2, y: 0})
	if from != expected {
		t.Errorf("Expected: %+v, actual: %+v", expected, from)
	}

	// Same column
	from = point{x: 0, y: 0}
	expected = point{x: 0, y: 1}
	from.closeTo(point{x: 0, y: 2})
	if from != expected {
		t.Errorf("Expected: %+v, actual: %+v", expected, from)
	}

	from = point{x: 0, y: 0}
	expected = point{x: 0, y: -1}
	from.closeTo(point{x: 0, y: -2})
	if from != expected {
		t.Errorf("Expected: %+v, actual: %+v", expected, from)
	}

	// No same row, no same col - top
	from = point{x: 0, y: 0}
	expected = point{x: 1, y: -1}
	from.closeTo(point{x: 1, y: -2})
	if from != expected {
		t.Errorf("Expected: %+v, actual: %+v", expected, from)
	}

	from = point{x: 0, y: 0}
	expected = point{x: -1, y: -1}
	from.closeTo(point{x: -1, y: -2})
	if from != expected {
		t.Errorf("Expected: %+v, actual: %+v", expected, from)
	}

	// No same row, no same col - down
	from = point{x: 0, y: 0}
	expected = point{x: 1, y: 1}
	from.closeTo(point{x: 1, y: 2})
	if from != expected {
		t.Errorf("Expected: %+v, actual: %+v", expected, from)
	}

	from = point{x: 0, y: 0}
	expected = point{x: -1, y: 1}
	from.closeTo(point{x: -1, y: 2})
	if from != expected {
		t.Errorf("Expected: %+v, actual: %+v", expected, from)
	}

	// No same row, no same col - right
	from = point{x: 0, y: 0}
	expected = point{x: 1, y: 1}
	from.closeTo(point{x: 2, y: 1})
	if from != expected {
		t.Errorf("Expected: %+v, actual: %+v", expected, from)
	}

	from = point{x: 0, y: 0}
	expected = point{x: 1, y: -1}
	from.closeTo(point{x: 2, y: -1})
	if from != expected {
		t.Errorf("Expected: %+v, actual: %+v", expected, from)
	}

	// No same row, no same col - left
	from = point{x: 0, y: 0}
	expected = point{x: -1, y: 1}
	from.closeTo(point{x: -1, y: 2})
	if from != expected {
		t.Errorf("Expected: %+v, actual: %+v", expected, from)
	}

	from = point{x: 0, y: 0}
	expected = point{x: -1, y: -1}
	from.closeTo(point{x: -1, y: -2})
	if from != expected {
		t.Errorf("Expected: %+v, actual: %+v", expected, from)
	}
}
