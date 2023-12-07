# Remove Duplicates from Sorted List II

https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/

## Summary

 - Use 2 pointers:
	 - Last is pointer that's before duplicated nodes.
	 - Current is pointer will be moved forward to continues checking.
 - If the pointer current.Next is same value with current, we loop until current.Next is differrent with current. It help to skip duplicated items. Last will link to new current.Next that's different with current.
 - If the pointer current.Next is NOT same value with current, move forward the Last because we surely current.Next is not duplicated of current.

## Code

```go
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func deleteDuplicates(head *ListNode) *ListNode {
    if head == nil {
        return nil
    }
    
    paddingHead := &ListNode{
        Val: 0,
        Next: head,
    }
    last := paddingHead
    current := head
    
    for current != nil {
        if current.Next != nil && current.Val == current.Next.Val {
            for current.Next != nil && current.Val == current.Next.Val {
                current = current.Next
            }
            last.Next = current.Next
        } else {
            last = last.Next
        }
        current = current.Next
    }

    return paddingHead.Next
}
```

```rust
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut writingIndex = 0;
        for readingIndex in 0..nums.len() {
            if nums[readingIndex] != nums[writingIndex] {
                nums[readingIndex] = nums[writingIndex];
                writingIndex+=1;
            }
        }
        (writingIndex + 1) as i32
    }
}
```