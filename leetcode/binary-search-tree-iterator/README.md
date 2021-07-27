# Binary Search Tree Iterator

https://leetcode.com/problems/binary-search-tree-iterator

## Summary

- Use stack to get all items of lists
- Sort array and store it internally as a stack
- HasNext() check if internal sorted stack's empty
- Next() pop the stack

## Code

```go
/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
type BSTIterator struct {
    inner []int
}

func Constructor(root *TreeNode) BSTIterator {
    inner := []int{}
    stack := []*TreeNode{root}
    for len(stack) > 0 {
        item := stack[len(stack) - 1]
        stack = stack[:len(stack)-1]
        inner = append(inner, item.Val)
        if item.Left != nil {
            stack = append(stack, item.Left)
        }
        if item.Right != nil {
            stack = append(stack, item.Right)
        }
    }
    sort.Ints(inner)
    return BSTIterator{inner}
}


func (this *BSTIterator) Next() int {
    r := this.inner[0]
    this.inner = this.inner[1:]
    return r
}


func (this *BSTIterator) HasNext() bool {
    return len(this.inner) > 0 
}


/**
 * Your BSTIterator object will be instantiated and called as such:
 * obj := Constructor(root);
 * param_1 := obj.Next();
 * param_2 := obj.HasNext();
 */
```
