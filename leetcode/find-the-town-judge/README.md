# Find the Town Judge

https://leetcode.com/problems/find-the-town-judge/

## Summary

 Everybody in the town have two main attribute:
  - Attribute 1: the person voted someone else.
  - Attribute 2: Number of other person who vote to.

So bases on the requirement, the town judge is a person who have:
 - Attribute 1 = 0
 - Attribute 2 = total town people - 1 (except town judge)

```go
func findJudge(n int, trust [][]int) int {
    attr1 := make([]bool, n, n)
    attr2 := make([]int, n, n)
    for _, t := range trust {
        attr1[t[0] - 1] = true
        attr2[t[1] - 1]++
    }
    
    
    for index, trustSomeoneElse := range attr1 {
        if !trustSomeoneElse && attr2[index] == n -1 {
            return index + 1
        }
    }
    return -1
}
```
