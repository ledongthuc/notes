# House Robber II

 - Dynamic programming
 - Best properties you got from 0 to N is maximum of:
   - Best properties you got from 1 to N
   - Best properties you got from 0 to N-1

```go
func rob(nums []int) int {
    if len(nums) == 0 {
        return 0;
    }
    if len(nums) == 1 {
        return nums[0]
    }
    if len(nums) == 2 {
        return max(nums[0], nums[1])
    }

    return max(sub(nums[:len(nums)-1]), sub(nums[1:]))
}

func sub(nums []int) int {
    var currMax, last, tmp int
    for i := range nums {
        tmp = currMax
        currMax = max(currMax, last + nums[i])
        last = tmp
    }
    return currMax
}

func max(a, b int) int {
    if a > b {
        return a
    }
    return b
}
```
