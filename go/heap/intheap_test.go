package heap

import (
	"reflect"
	"testing"
)

func TestInitHeapMinInt(t *testing.T) {
	tests := []struct {
		name string
		arr  []int
		want *HeapMinInt
	}{
		{
			name: "Test",
			arr:  []int{2, 1, 5},
			want: &HeapMinInt{1, 2, 5},
		},
		{
			name: "Test 2",
			arr:  []int{2, 7, 4, 1, 8, 1},
			want: &HeapMinInt{1, 2, 1, 7, 8, 4},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := InitHeapMinInt(tt.arr); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("InitHeapMinInt() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestHeapMinInt_Pop(t *testing.T) {
	h := InitHeapMinInt([]int{2, 7, 4, 1, 8, 1})
	expects := []int{1, 1, 2, 4, 7, 8}
	for index, expect := range expects {
		if item := h.Pop(); item != expect {
			t.Errorf("HeapMinInt.Pop()[%d] = %v, want %v", index+1, item, expect)
		}
	}
}

func TestInitHeapMaxInt(t *testing.T) {
	tests := []struct {
		name string
		arr  []int
		want *HeapMaxInt
	}{
		{
			name: "Test",
			arr:  []int{2, 1, 5},
			want: &HeapMaxInt{5, 1, 2},
		},
		{
			name: "Test 2",
			arr:  []int{2, 7, 4, 1, 8, 1},
			want: &HeapMaxInt{8, 7, 4, 1, 2, 1},
		},
		{
			name: "Test 3",
			arr:  []int{10, 5, 4, 10, 3, 1, 7, 8},
			want: &HeapMaxInt{10, 10, 7, 8, 3, 1, 4, 5},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := InitHeapMaxInt(tt.arr); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("InitHeapMaxInt() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestHeapMaxInt_Pop(t *testing.T) {
	h := InitHeapMaxInt([]int{2, 7, 4, 1, 8, 1})
	expects := []int{8, 7, 4, 2, 1, 1}
	for index, expect := range expects {
		if item := h.Pop(); item != expect {
			t.Errorf("HeapMaxInt.Pop()[%d] = %v, want %v", index+1, item, expect)
		}
	}
}

func TestMash2TopAndPushRemaning(t *testing.T) {
	h := InitHeapMaxInt([]int{2, 7, 4, 1, 8, 1})
	expects := []int{1, 2, 1, 0, 1}
	i := 0
	for len(*h) > 1 {
		expect := expects[i]
		item := h.Mash2TopAndPushRemaning()
		if item != expect {
			t.Errorf("HeapMaxInt.Mash2TopAndPushRemaning()[%d] = %v, want %v", i, item, expect)
		}
		i++
	}
}

func TestMash2TopAndPushRemaning2(t *testing.T) {
	h := InitHeapMaxInt([]int{9, 3, 2, 10})
	expects := []int{1, 1, 0}
	i := 0
	for len(*h) > 1 {
		expect := expects[i]
		item := h.Mash2TopAndPushRemaning()
		if item != expect {
			t.Errorf("HeapMaxInt.Mash2TopAndPushRemaning()[%d] = %v, want %v", i, item, expect)
		}
		i++
	}
}

/* Additional function */
func TestMash2TopAndPushRemaning3(t *testing.T) {
	h := InitHeapMaxInt([]int{10, 5, 4, 10, 3, 1, 7, 8})
	expects := []int{0, 1, 1, 2, 1, 0}
	i := 0
	for len(*h) > 1 {
		expect := expects[i]
		item := h.Mash2TopAndPushRemaning()
		if item != expect {
			t.Errorf("HeapMaxInt.Mash2TopAndPushRemaning()[%d] = %v, want %v", i, item, expect)
		}
		i++
	}
}
