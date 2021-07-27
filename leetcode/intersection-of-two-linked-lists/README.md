# Intersection of Two Linked Lists

https://leetcode.com/problems/intersection-of-two-linked-lists

## Summary

- Intersection lists only merged right sides to cut off the right node to ensure they are the same length before comparing.
- After making two lists of the same length, use two pointers to check if they are the same node item sequence.

## Code

```go
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func getIntersectionNode(headA, headB *ListNode) *ListNode {
    if headA == nil || headB == nil {
        return nil
    }
    
    lA, lB := count(headA), count(headB)
    
    for lA > lB {
        headA = headA.Next
        lA--
    }
    for lA < lB {
        headB = headB.Next
        lB--
    }
    
    for headA != headB {
        if headA != nil {
            headA = headA.Next            
        }
        if headB != nil {
            headB = headB.Next            
        }
    }
    
    return headA
}

func count(node *ListNode) (i int) {
    for node != nil {
        i++
        node = node.Next
    }
    return i
}
```
