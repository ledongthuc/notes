package main

import (
	"container/heap"
	"fmt"
)

type PriorityQueue []*PriorityItem

type PriorityItem struct {
	Priority int
	Value    int
}

func (p PriorityQueue) Len() int      { return len(p) }
func (p PriorityQueue) Swap(i, j int) { p[i], p[j] = p[j], p[i] }

func (p PriorityQueue) Less(i, j int) bool {
	return p[i].Priority > p[j].Priority
}

func (p *PriorityQueue) Push(item interface{}) {
	priorityItem, valid := item.(*PriorityItem)
	if !valid {
		return
	}
	*p = append(*p, priorityItem)
}

func (p *PriorityQueue) Pop() interface{} {
	old := *p
	n := len(old)
	item := old[n-1]
	old[n-1] = nil
	*p = old[0 : n-1]
	return item
}

func (p *PriorityQueue) Debug() {
	for index, item := range *p {
		fmt.Printf("[%d] priority: %d, value: %d\n", index+1, item.Priority, item.Value)
	}
}

func main() {
	p := &PriorityQueue{}
	heap.Push(p, &PriorityItem{Priority: 1, Value: 5})
	heap.Push(p, &PriorityItem{Priority: 7, Value: 3})
	heap.Push(p, &PriorityItem{Priority: 3, Value: 2})
	heap.Push(p, &PriorityItem{Priority: 2, Value: 100})
	heap.Push(p, &PriorityItem{Priority: 4, Value: 200})
	heap.Push(p, &PriorityItem{Priority: 8, Value: 800})
	heap.Push(p, &PriorityItem{Priority: 5, Value: 500})
	heap.Push(p, &PriorityItem{Priority: 10, Value: 1000})
	heap.Push(p, &PriorityItem{Priority: 9, Value: 900})
	heap.Push(p, &PriorityItem{Priority: 6, Value: 600})

	fmt.Println("After pushing")
	p.Debug()

	fmt.Println("Popping 1: ", heap.Pop(p))
	fmt.Println("Popping 2: ", heap.Pop(p))
	fmt.Println("Popping 3: ", heap.Pop(p))
	p.Debug()
}
