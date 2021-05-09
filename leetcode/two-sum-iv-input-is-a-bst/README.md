# Two Sum IV - Input is a BST

https://leetcode.com/problems/two-sum-iv-input-is-a-bst/

## Summary
 - To check any sum 2 nodes equal to `k`, we should walk through all nodes of tree.
	 - While walking through the tree, save node's value to a cache while walking through the tree
	 - While walking through the tree, check if any `remaining = k - node's value` available in the cache, then meeting the problem requirement

```go
func pathSum(root *TreeNode, targetSum int) int {
    return walk(root, targetSum, []int{})
}

func walk(node *TreeNode, targetSum int, branch []int) int {
    if node == nil {
        return 0
    }
    var counter int
    
    branch = append(branch, node.Val)
    var checkingSum int
    for i := len(branch) - 1; i >= 0; i-- {
        checkingSum+= branch[i]
        if checkingSum == targetSum {
            counter++
        }
    }
    return counter + walk(node.Left, targetSum, branch) + walk(node.Right, targetSum, branch)
}
```

## Recursive reduction

 - Use a stack to reduce the recursion of tree walking
	 - Push children of a node to stack
	 - Loop until stack empty
	 - Pull from stack and compare with cached nodes values that their sum are `k`

```go
func pathSum(root *TreeNode, targetSum int) int {
    if root == nil {
        return  0
    }
    
    var totalMatches int
    s := stack{
        stackItem{root, []int{root.Val}},
    }
    
    for {
        if len(s) == 0 {
            return totalMatches
        }
        
        item := s.Pull()
        if item.node == nil {
            continue
        }
        node := item.node
        branch := item.branch
        
        var checkingSum int
        for i := len(branch) - 1; i >= 0; i-- {
            checkingSum+= branch[i]
            if checkingSum == targetSum {
                totalMatches++
            }
        }
        
        
        if node.Left != nil {
            leftBranch := make([]int, len(branch), len(branch))
            copy(leftBranch, branch)
            leftBranch = append(leftBranch, node.Left.Val)
            
            s.Push(stackItem{node.Left, leftBranch})
        }
        
        
        if node.Right != nil {
            rightBranch := make([]int, len(branch), len(branch))
            copy(rightBranch, branch)
            rightBranch = append(rightBranch, node.Right.Val)
            s.Push(stackItem{node.Right, rightBranch})
        }
    }
    return totalMatches
}


type stack []stackItem
type stackItem struct {
    node *TreeNode
    branch []int
}

func (s *stack) Push(n1 stackItem) {
    (*s) = append((*s), n1)
}

func (s *stack) Pull() (stackItem) {
    if len(*s) == 0 {
        return stackItem{}
    }
    
    result := (*s)[len(*s)-1]
    (*s) = (*s)[:len(*s)-1]
    return result
}
```
