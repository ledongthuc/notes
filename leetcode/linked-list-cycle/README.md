# Linked List Cycle

https://leetcode.com/problems/linked-list-cycle

## Summary

- Create a fake node item that's used to detect the cycle list.
- Go through the list from the head. With the passed node, we assign the fake node item as their following link.
- Any node when checking has the next item is the fake node, which means we went through it already. The list cycles.

## Code

```go
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func hasCycle(head *ListNode) bool {
    if head == nil {
        return false
    }
    curr := head
    next := curr.Next
    
    checking := &ListNode{}
    
    for next != nil {
        curr.Next = checking
        if next == checking {
            return true
        }
        
        curr = next
        next = curr.Next
    }
    return false
}
```
