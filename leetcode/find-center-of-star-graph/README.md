# Find Center of Star Graph

https://leetcode.com/problems/find-center-of-star-graph/

## Summary

The key to solving this problem is to recognize that the center node of the star graph will have multiple connections to other nodes.

Therefore, you should look for a node that appears more than once in the input; that node is the center of the star graph.

## Code

```go
func findCenter(edges [][]int) int {
    centerChecker := [100001]bool{}
    for _, edge := range edges {
        if (centerChecker[edge[0]]) {
            return edge[0]
        }
        if (centerChecker[edge[1]]) {
            return edge[1]
        }
        centerChecker[edge[0]] = true
        centerChecker[edge[1]] = true
    }
    return -1
}
```


```go
func findCenter(edges [][]int) int {
    if edges[0][0] == edges[1][0] || edges[0][0] == edges[1][1] {
        return edges[0][0]
    }
    return edges[0][1]
}
```
