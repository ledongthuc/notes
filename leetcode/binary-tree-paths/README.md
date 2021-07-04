# Binary Tree Paths

https://leetcode.com/problems/binary-tree-paths/

## Summary

 - Use deep first search, collect leaves and put to final result

![enter image description here](https://upload.wikimedia.org/wikipedia/commons/thumb/7/7f/Depth-First-Search.gif/220px-Depth-First-Search.gif)

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
func binaryTreePaths(root *TreeNode) []string {
    return gogogo(root, fmt.Sprintf("%d", root.Val))
}

func gogogo(node *TreeNode, pathValue string) []string {
    if node.Left == nil && node.Right == nil {
        return []string{pathValue}
    }
    
    result := []string{}
    if node.Left != nil {
        result = append(result, gogogo(node.Left, fmt.Sprintf("%s->%d", pathValue, node.Left.Val))...)
    }
    if node.Right != nil {
        result = append(result, gogogo(node.Right, fmt.Sprintf("%s->%d", pathValue, node.Right.Val))...)
    }
    return result
}
```
