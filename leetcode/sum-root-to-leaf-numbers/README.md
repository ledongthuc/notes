# Sum Root to Leaf Numbers

https://leetcode.com/problems/sum-root-to-leaf-numbers/

## Summary

 - Use deep first search, collect leaves and put to final result
 - Each node from root  multiple with  10

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
func sumNumbers(root *TreeNode) int {
    arr := collectNumber(root, root.Val)
    var result int
    for _, item := range arr {
        result += item
    }
    return result
}

func collectNumber(node *TreeNode, value int) []int {
    if node.Left == nil && node.Right == nil {
        return []int{value}
    }
    
    var result []int
    if node.Left != nil {
        result = append(result, collectNumber(node.Left, value * 10 + node.Left.Val)...)
    }
    if node.Right != nil {
        result = append(result, collectNumber(node.Right, value * 10 + node.Right.Val)...)
    }
    return result
}
```
