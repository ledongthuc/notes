# Min Stack

https://leetcode.com/problems/min-stack

## Summary

Building stack with two solutions:
 - Array
 - Linked list

## Array

 -  Stack stores data in an array, internally.
 - An array item contains latest value and mininum value from starting until that latest value
 - Push = append array with 
	 - pushed value
	 - mininum value between pushed value and last minumum value
 - Pop = Remove latest item
 - Top = return latest item's value
 - Get min = return latest item's miniumum value

```go
type StackItem struct {
    Value int
    MinItemInThisTime int
}

type MinStack struct {
    InternalStack []StackItem
}

func Constructor() MinStack {
    return MinStack{InternalStack: make([]StackItem,0,0)}
}

func (this *MinStack) Push(x int)  {
    if len(this.InternalStack) == 0 {
        this.InternalStack = append(this.InternalStack, StackItem{ Value: x, MinItemInThisTime: x })
    } else {
        this.InternalStack = append(this.InternalStack, StackItem{ Value: x, MinItemInThisTime: min(this.GetMin(), x) })
    }
}

func (this *MinStack) Pop()  {
    this.InternalStack = this.InternalStack[:len(this.InternalStack) - 1]
}

func (this *MinStack) Top() int {
	return this.InternalStack[len(this.InternalStack) - 1].Value
}


func (this *MinStack) GetMin() int {
    return this.InternalStack[len(this.InternalStack) - 1].MinItemInThisTime
}

func min(v1, v2 int) int{
    if v1 > v2 {
        return v2
    }
    return v1
}
```

## Linked list

 -  Stack stores data in a linked list, internally.
 - Stack keeps pointer to latest item of linked list. 
 - Linked list item refers to previous item.
 - An linked list item keeps latest value and mininum value from starting until that latest value
 - Push = Add new item to linked list. Stack keep new linked list item
 - Pop = Remove latest item. Stack keep previous linked list item.
 - Top = return latest item's value
 - Get min = latest last item's miniumum value

```go
type LinkedListItem struct {
    value int
    minValue int
    previous *LinkedListItem
}

type MinStack struct {
    latest *LinkedListItem
}

func Constructor() MinStack {
    return MinStack{}
}


func (this *MinStack) Push(val int)  {
    if this.latest == nil {
        this.latest = &LinkedListItem{value: val, minValue: val}
        return
    }
    
    this.latest = &LinkedListItem{
        value: val,
        minValue: min(this.latest.minValue, val),
        previous: this.latest,
    }
}

func (this *MinStack) Pop()  {
    fmt.Println("")
    this.latest = this.latest.previous
}

func (this *MinStack) Top() int {
    return this.latest.value
}


func (this *MinStack) GetMin() int {
    return this.latest.minValue
}

func min(v1, v2 int) int{
    if v1 > v2 {
        return v2
    }
    return v1
}
```
