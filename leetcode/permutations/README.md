# Permutations

https://leetcode.com/problems/permutations/

## Summary
 - Based on backtracking (https://en.wikipedia.org/wiki/Backtracking)
 - Two solutions:
	 - Build tree then collect results.
	 - Swap elements.

## Tree
- Use elements as a root node of a tree, remaining elements are building the branches and leafs.
- With each remaining elements of a tree, attach it to root element to have branches.
- Each branches continue to use remaining elements (after remove root and first branch) to build until out of remaining elements.
 - Collect all leafs of the trees as result of permutations.

![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/permutations/Permutations-build%20tree%20%28step%29.png)
![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/permutations/Permutations-build%20tree%20%28full%29.png)

## Tree - example:

Following solution, you can combine step "build tree" and "harvest leaves" to once for performance trick.
But It's not fun with me, so let's put them seperatedly for single responsibility.

```
func permute(nums []int) [][]int {
    root := &Node{ Value: []int{}}
    buildTree(root, nums)
    return harvest(root)
}

type Node struct {
    Children []*Node
    Value []int
}

func (n *Node) composeBranchValue(value int) []int {
    branchValue := make([]int, len(n.Value))
    copy(branchValue, n.Value)
    return append(branchValue, value)
}

func buildTree(parent *Node, branchValues []int) {
    if len(branchValues) == 0 {
        return
    }
    
    for index := range branchValues {
        remaning, branchValue := cut(branchValues, index)
        
        child := &Node{Value: parent.composeBranchValue(branchValue)}
        parent.Children = append(parent.Children, child)
        
        buildTree(child, remaning)
    }
}

func harvest(n *Node) (leaves [][]int) {
    if len(n.Children) == 0 {
        return [][]int{n.Value}
    }
    
    for _, child := range n.Children {
        leaves = append(leaves, harvest(child)...)
    }
    
    return leaves
}

func cut(arr []int, index int) (remainingArr []int, item int) {
    item  = arr[index]
    remainingArr = append([]int{}, arr[:index]...)
    return append(remainingArr, arr[index+1:]...), item
}
```

## Swap elements
 - Swap first element with remaining elements.
 - Lock first element and continue swapping remaining parts.
 - Continue swapping until we don't have remianing elements.
 - Collect all cases as result of permutations.

![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/permutations/Permutations-swap.png)

## Swap elements - example 1

Old solution that I don't remember when I did it. But I don't like it, so re-implement it with example 2

```
func permute(nums []int) [][]int {
    return permuteP([][]int{[]int{}}, nums) 
}

func permuteP(current [][]int, rest []int) (result [][]int) {
    if len(rest) == 0 {
        return current
    }
    for cIndex, _ := range current {
        for rIndex, _ := range rest {
            temp := append(current[cIndex], rest[rIndex])
            newRest := append([]int{}, rest[:rIndex]...)
            newRest = append(newRest, rest[rIndex+1:]...)
            result = append(result, permuteP([][]int{temp}, newRest)...)
        }
    }
    
    return result
}
```

## Swap elements - example 2

```
func permute(nums []int) [][]int {
    return permuteR([]int{}, nums)
}

func permuteR(lockedArr []int, remainingArr []int)  [][]int {
    if len(remainingArr) == 0 {
        return [][]int{lockedArr}
    }
    
    result := [][]int{}
    for lockIndex := 0; lockIndex < len(remainingArr); lockIndex++ {
        lockedItem, remainingArr := lockItem(remainingArr, lockIndex)
        newLockedArr := newLockedItem(lockedArr, lockedItem)
        result = append(result, permuteR(newLockedArr, remainingArr)...)
    }
    return result
}

func lockItem(arr []int, lockIndex int) (lockedItem int, remainingArr []int) {
    lockedItem  = arr[lockIndex]
    remainingArr = append([]int{}, arr[:lockIndex]...)
    return lockedItem, append(remainingArr, arr[lockIndex+1:]...)
}

func newLockedItem(arr []int, lockedItem int) []int {
    return append(append([]int{}, arr...), lockedItem)
}
```
