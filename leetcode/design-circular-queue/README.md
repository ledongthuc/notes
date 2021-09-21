# Design Circular Queue

https://leetcode.com/problems/design-circular-queue/

## Summary

 - Use an internal array as data storage, 2 pointers to track the starting and ending of queue items.
 - Implement a method that allows to circularly increase if the pointer is reach capacity of the internal array. It will move back to starting of array (index = 0)
 - Enqueuing an item: increase the actual size of queue, circularly increase the ending pointer (increase one item) and set new value to ending pointer. Enqueuing is failed if actual size of the queue equals capacity of the internal array.
 - Dequeuing an item, decrease the actual size of queue, circularly increase the starting pointer (remove first item). Dequeuing is failed if actual size of the queue equals 0.

## Code

```go
type MyCircularQueue struct {
    size int
    capacity int
    inner []int
    startIndex int
    endIndex int
}


func Constructor(k int) MyCircularQueue {
    return MyCircularQueue{
        size: 0,
        capacity: k,
        inner: make([]int, k),
        startIndex: 0,
        endIndex: -1, // to make first element will be in 0
    }
}


func (this *MyCircularQueue) EnQueue(value int) bool {
    if this.IsFull() {
        return false
    }
    
    this.size++
    this.endIndex = this.CircularIncreasement(this.endIndex)
    this.inner[this.endIndex] = value
    return true
}


func (this *MyCircularQueue) DeQueue() bool {
    if this.IsEmpty() {
        return false
    }
    
    this.size--
    this.startIndex = this.CircularIncreasement(this.startIndex)
    return true
}


func (this *MyCircularQueue) Front() int {
    if this.size == 0 {
        return -1
    }
    return this.inner[this.startIndex]
}


func (this *MyCircularQueue) Rear() int {
    if this.size == 0 {
        return -1
    }
    return this.inner[this.endIndex]
}


func (this *MyCircularQueue) IsEmpty() bool {
    return this.size == 0
}


func (this *MyCircularQueue) IsFull() bool {
    return this.size == this.capacity
}

func (this *MyCircularQueue) CircularIncreasement(i int) int {
    if i == this.capacity-1{
        return 0
    }
    return i+1
}
```

