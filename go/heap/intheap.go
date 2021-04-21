package heap

type HeapMaxInt []int

func InitHeapMaxInt(arr []int) *HeapMaxInt {
	heap := HeapMaxInt(arr)
	for i := len(heap)/2 - 1; i >= 0; i-- {
		heap.heapifyDown(i)
	}
	return &heap
}

func (h *HeapMaxInt) Pop() int {
	item := (*h)[0]
	(*h)[0] = (*h)[len(*h)-1]
	(*h) = (*h)[:len(*h)-1]
	h.heapifyDown(0)
	return item
}

func (h *HeapMaxInt) Push(item int) {
	(*h) = append(*h, item)
	h.heapifyUp(len(*h) - 1)
}

func (h *HeapMaxInt) heapify(i int) {
	for i := len(*h)/2 - 1; i >= 0; i-- {
		(*h).heapifyDown(i)
	}
}

func (h *HeapMaxInt) heapifyDown(i int) {
	max := i
	left := 2*i + 1
	right := 2*i + 2

	if left < len(*h) && (*h)[left] > (*h)[max] {
		max = left
	}

	if right < len(*h) && (*h)[right] > (*h)[max] {
		max = right
	}

	if max != i {
		(*h)[i], (*h)[max] = (*h)[max], (*h)[i]
		h.heapifyDown(max)
	}
}

func (h *HeapMaxInt) heapifyUp(i int) {
	parent := (i - 1) / 2
	if (*h)[i] <= (*h)[parent] {
		return
	}
	(*h)[i], (*h)[parent] = (*h)[parent], (*h)[i]
	h.heapifyUp(parent)
}

func (h *HeapMaxInt) Mash2TopAndPushRemaning() int {
	if len(*h) == 0 {
		return 0
	}

	if len(*h) == 1 {
		return (*h)[0]
	}

	first := h.Pop()
	second := h.Pop()
	remaining := first - second
	if remaining > 0 {
		h.Push(remaining)
	}
	return remaining
}
