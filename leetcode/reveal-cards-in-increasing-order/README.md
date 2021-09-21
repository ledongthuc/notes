# Reveal Cards In Increasing Order

https://leetcode.com/problems/reveal-cards-in-increasing-order/

## Summary

 - Create a simulate sorted deck index, use it to map to sorted deck.
 - Go to loop with each step:
	 - Reveale top card
	 - Check next card and move it to bottom
- Use a Queue to simulate sorted deck index and managed index.
- Index from queue pop card is index of reveale card.

## Code

```go
func deckRevealedIncreasing(deck []int) []int {
    // Sort the deck
    sort.Ints(deck)
    
    // Simulate with sorted deck
    queue := QueueConstructor() 
    for i := 0; i < len(deck); i++ {
        queue.Push(i)
    }
    
    // Reveale top card, check next card and move to bottom 
    result := make([]int, len(deck))
    for index, card := range deck {
        result[queue.Pop()] = card
        if index != len(deck) - 1 {
            queue.Push(queue.Pop())
        }
    }
    return result
}

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

