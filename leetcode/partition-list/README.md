# Partition List

https://leetcode.com/problems/partition-list

## Summary

We need pointers:
 - `current` is current item checking
 - `previous` is previous item of current checking item
 - `lastFirstPartition` is last item of first part (aka. partition of item that's less than x)

Go through item from linked list, action depends on scenario:

### 1.  Value is greater than or equal x
- Do nothing

### 2. Value is less than x, and it's head
- Track current item is `lastFirstPartition`

### 3. Value is less than x, and previous is first partition (aka. partition of item that's less than x)
 - Move `lastFirstPartition` to current item

### 4. Value is less than x, and didn't see any item that's less than x yet
 - Move current item to head. Last head will be second.
 - Track current item (now is head) is `lastFirstPartition`

### 5. Value is less than x, and we have any items that's less than x already
 - Move current item to next of `lastFirstPartition`. Current item will be new `lastFirstPartition`

## Code

```go
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func partition(head *ListNode, x int) *ListNode {
    current := head
    var lastFirstPartition *ListNode
    var previous *ListNode
    
    for current != nil {
        switch {
            case current.Val >= x:
                // do nothing here
            case previous == nil, lastFirstPartition != nil && lastFirstPartition.Next == current:
                lastFirstPartition = current
            case lastFirstPartition == nil:
                previous.Next = current.Next
                previousHead := head
                head = current
                head.Next = previousHead
                lastFirstPartition = current
            case lastFirstPartition != nil:
                previous.Next = current.Next
                current.Next = lastFirstPartition.Next
                lastFirstPartition.Next = current
                lastFirstPartition = current
                current = previous
        }
        
        previous = current
        current = current.Next
    }
    return head
}
```
