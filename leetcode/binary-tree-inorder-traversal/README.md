# Binary Tree Inorder Traversal

https://leetcode.com/problems/binary-tree-inorder-traversal

## Summary

 - Recursively, go through the tree to collect values with priority:
	 - Left branch
	 - Current node
	 - Right branch

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
func inorderTraversal(root *TreeNode) []int {
    if root == nil {
        return nil
    }
    result := []int{}
    return inorderTraversalInternal(root, result)
}

func inorderTraversalInternal(item *TreeNode, result []int) []int {
    if item.Left != nil {
        result = inorderTraversalInternal(item.Left, result)
    }
    
    result = append(result, item.Val)
    
    if item.Right != nil {
        result = inorderTraversalInternal(item.Right, result)
    }
    
    return result
}
```

