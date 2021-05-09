# Path Sum III

https://leetcode.com/problems/path-sum-iii/

## Summary

 - To count number of paths where the sum of the values along the path equals `k`, we go through all branches from root.
	 - Each branches from root, we walk from root to leave to build the branch.
	 - Each node are added to the branch while building, we can sum its value with parents sequence to count any new pathes that have their sum equals `k`

![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/path-sum-iii/PathSumIII-branches.png)

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
	 - Pull node from stack and run to check their sum branches value from root

```go
func findTarget(root *TreeNode, k int) bool {
    checking := map[int]struct{}{}
    s := stack{root}
    for {
        if len(s) == 0 {
            return false
        }
        
        node := s.Pull()
        if _, existed := checking[k - node.Val]; existed {
            return true
        }
        checking[node.Val] = struct{}{}
        s.Push(node.Left, node.Right)
    }
    return false
}

type stack []*TreeNode

func (s *stack) Push(n1, n2 *TreeNode) {
    if n1 != nil {
        (*s) = append((*s), n1)
    }
    if n2 != nil {
        (*s) = append((*s), n2)
    }
}

func (s *stack) Pull() (*TreeNode) {
    if len(*s) == 0 {
        return nil
    }
    
    result := (*s)[len(*s)-1]
    (*s) = (*s)[:len(*s)-1]
    return result
}
```
