# Same Tree

https://leetcode.com/problems/same-tree

## Summary

 - Recursively, compare:
 	 - Current node's values
	 - Left branches
	 - Right branches

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
func isSameTree(p *TreeNode, q *TreeNode) bool {
	if p == nil && q == nil {
		return true
	}
	if p == nil || q == nil {
		return false
	}
	if p.Val != q.Val {
        return false
	}
	return isSameTree(p.Left, q.Left) && isSameTree(p.Right, q.Right)
}
```
