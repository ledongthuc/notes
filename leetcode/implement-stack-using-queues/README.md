# Implement Stack using Queues

https://leetcode.com/problems/implement-stack-using-queues/

## Summary

 - Use 2 internal queues, called main queue and support queue.
 - When push to the stack, we push it to the main queue.
 - When pop from the stack, we pull all items from main queue and push to support queue. Last item of main queue is popped stack, we don't need to push it to support queue. Now, support queue become to be a main queue.

## Code

```go
type MyStack struct {
    mainQueue    MyQueue
    supportQueue MyQueue
}

/** Initialize your data structure here. */
func Constructor() MyStack {
    return MyStack{
        mainQueue: QueueConstructor(),
        supportQueue: QueueConstructor(),
    }
}

/** Push element x onto stack. */
func (this *MyStack) Push(x int)  {
    this.mainQueue.Push(x)
}

/** Removes the element on top of the stack and returns that element. */
func (this *MyStack) Pop() int {
    var v int
    for {
        v = this.mainQueue.Pop()
        if this.mainQueue.Empty() {
            break
        }
        this.supportQueue.Push(v)
    }
    this.mainQueue, this.supportQueue = this.supportQueue, this.mainQueue
    return v
}

/** Get the top element. */
func (this *MyStack) Top() int {
    var v int
    for !this.mainQueue.Empty() {
        v = this.mainQueue.Pop()
        this.supportQueue.Push(v)
    }
    this.mainQueue, this.supportQueue = this.supportQueue, this.mainQueue
    return v
}

/** Returns whether the stack is empty. */
func (this *MyStack) Empty() bool {
    return this.mainQueue.Empty()
}


/********* Implement Queue **********/

type MyQueue struct {
    inner []int
}

/** Initialize your data structure here. */
func QueueConstructor() MyQueue {
    return MyQueue {
        inner: make([]int, 0),
    }
}

/** Push element x to the back of queue. */
func (this *MyQueue) Push(x int)  {
    this.inner = append(this.inner, x)
}

/** Removes the element from in front of queue and returns that element. */
func (this *MyQueue) Pop() int {
    popValue := this.inner[0]
    this.inner = this.inner[1:]
    return popValue
}

/** Get the front element. */
func (this *MyQueue) Peek() int {
    return this.inner[0]
}

/** Returns whether the queue is empty. */
func (this *MyQueue) Empty() bool {
    return len(this.inner) == 0
}
```

