# 3Sum

https://leetcode.com/problems/3sum/

## Summary

- Break the big problem to smaller problems
- Big problem: find `i, j, k` with `i + j + k = 0`
- With each `i` as item in the array, find `j, k` with `j + k = -i`
- With each `i, k` as 2 items in the array, find `k` with `k = -i - j`

## Code

```go
func threeSum(nums []int) [][]int {
    sort.Ints(nums)
    
    r := [][]int{}
    var previous int
    for index, item := range nums {
        if index > len(nums) - 3 {
            break
        }
        if index > 0 && previous == item {
            continue
        }
        previous = item
        
        if nums[index+1] + nums[index+2] > -item {
            continue
        }
        
        r2 := twoSum(0 - item, nums[index+1:])
        for index2 := range r2 {
            r2[index2] = append(r2[index2], item)
        }
        r = append(r, r2...)
    }
    return r
}

func twoSum(total int, nums []int) [][]int {
    r := [][]int{}
    var previous int
    for index, item := range nums {
        if index > len(nums) - 2 {
            break
        }
        if index > 0 && previous == item {
            continue
        }
        previous = item
        remaining := total - item
        
        if nums[index+1] > remaining {
            break
        }
        
        if findItem(remaining, nums[index+1: len(nums)]) {
            r = append(r, []int{item, remaining})
        }
    }
    return r
}

func findItem(v int, nums []int) bool {
    for _, item := range nums {
        if item > v {
            return false
        }
        if item == v {
            return true
        }
    }
    return false
}
```
