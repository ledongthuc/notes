# 3Sum Closest

https://leetcode.com/problems/3sum-closest/

## Summary

- Break the big problem to smaller problems
- Big problem: find `min = i + j + k`
- With each `i` as item in the array, find `j, k` with `j + k = min - i`
- With each `i, k` as 2 items in the array, find `k` with `k = min - i - j`

## Code

```go
func threeSumClosest(nums []int, target int) int {
    sort.Ints(nums)
    
    min := int(math.MaxInt32)
    number := 0
    for i := 0; i < len(nums) - 2; i++ {
        for j := i+1; j < len(nums) - 1; j++ {
            for k := j+1; k < len(nums) - 0; k++ {
                sum := nums[i] + nums[j] + nums[k]
                remain := int(math.Abs(float64(sum - target)))
                if remain < min {
                    min = remain
                    number = sum
                }
            }
        }
    }
    return number
}
```
