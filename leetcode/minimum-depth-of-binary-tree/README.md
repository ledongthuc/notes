# Minimum Depth of Binary Tree

https://leetcode.com/problems/minimum-depth-of-binary-tree/

## Summary

Walk through nodes of the tree, with depth count variable tracking. Compare left and right of node and return shorter path depth.

Take care case node have no branches.

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
func minDepth(root *TreeNode) int {
    if root == nil {
        return 0
    }
    return walkNodeWithMinReturn(root, 1)
}

func walkNodeWithMinReturn(node *TreeNode, depth int) int  {
    if node.Left == nil && node.Right == nil {
        return depth
    }
    if node.Left == nil {
        return walkNodeWithMinReturn(node.Right, depth+1)
    }
    if node.Right == nil {
        return walkNodeWithMinReturn(node.Left, depth+1)
    }

    return min(
        walkNodeWithMinReturn(node.Left, depth+1), 
        walkNodeWithMinReturn(node.Right, depth+1),
    )
}

func minWithout1(a,b int) int {
    if a < b {
        return a
    }
    return b
}
```
