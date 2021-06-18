# Number of Good Pairs

https://leetcode.com/problems/number-of-good-pairs/

## Summary

 - With each item in the array, we increase `item's value counter`. Each item's value has a different counter.
 - We need a `total counter` that use to count pairs.
 If we found an existing item's value, we will increase the `total counter` = number of the existing item's value. The number of existed item's value is stored in `item's value counter`.

## Code

```go
func numIdenticalPairs(nums []int) int {
    if len(nums) < 2 {
        return 0
    }
    
    m := make([]int, 101, 101)
    count := 0
    for _, num := range nums {
        cNum := m[num]
        count += cNum
        m[num]++
    }
    return count
}
```
