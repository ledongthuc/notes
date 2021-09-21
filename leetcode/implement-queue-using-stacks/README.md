# Implement Queue using Stacks

https://leetcode.com/problems/implement-queue-using-stacks/

## Summary

 - Use 2 internal stacks, called main stack and support stack.
 - When push to the queue, we push it to the main stack.
 - When pop from the queue, we pull all items from main stack and push to support stack. Last item of main stack is popped queue, we don't need to push it to support stack. Finally, we need to pull all items from support stack, and push them to main stack to keep the order.

## Code

```go
type MyQueue struct {
    mainStack    MyStack
    supportStack MyStack
}

/** Initialize your data structure here. */
func Constructor() MyQueue {
    return MyQueue {
        mainStack: StackConstructor(),
        supportStack: StackConstructor(),
    }
}

/** Push element x to the back of queue. */
func (this *MyQueue) Push(x int)  {
    this.mainStack.Push(x)
}

/** Removes the element from in front of queue and returns that element. */
func (this *MyQueue) Pop() int {
    var v int
    for {
        v = this.mainStack.Pop()
        if this.mainStack.Empty() {
            break
        }
        this.supportStack.Push(v)
    }
    this.swapSupportToMain()
    return v
}

/** Get the front element. */
func (this *MyQueue) Peek() int {
    var v int
    for !this.mainStack.Empty() {
        v = this.mainStack.Pop()
        this.supportStack.Push(v)
    }
    this.swapSupportToMain()
    return v
}

func (this *MyQueue) swapSupportToMain() {
    for !this.supportStack.Empty() {
        this.mainStack.Push(this.supportStack.Pop())
    }
}

/** Returns whether the queue is empty. */
func (this *MyQueue) Empty() bool {
    return this.mainStack.Empty()
}

/********* Implement Stack **********/

type MyStack struct {
    inner []int
}

/** Initialize your data structure here. */
func StackConstructor() MyStack {
    return MyStack {
        inner: make([]int, 0),
    }
}

/** Push element x onto stack. */
func (this *MyStack) Push(x int)  {
    this.inner = append(this.inner, x)
}

/** Removes the element on top of the stack and returns that element. */
func (this *MyStack) Pop() int {
    v := this.inner[len(this.inner) - 1]
    this.inner = this.inner[0:len(this.inner) - 1]
    return v
}

/** Get the top element. */
func (this *MyStack) Top() int {
    return this.inner[len(this.inner) - 1]
}

/** Returns whether the stack is empty. */
func (this *MyStack) Empty() bool {
    return len(this.inner) == 0
}
```

