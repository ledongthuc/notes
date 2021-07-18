# House Robber

 - Dynamic programming
 - Best property of ropped houses you can do of N is maximum of:
   - Best property of ropped houses of N-1 (skip this turn)
   - Best property of ropped houses of N-2 + N (skip last turn)

```go
func rob(nums []int) int {
    length := len(nums)
    switch length {
    case 0:
        return 0
    case 1:
        return nums[0]
    default:
        bests := make([]int, length)
	    bests[0] = nums[0]
        bests[1] = max(nums[0], nums[1])
        for i := 2; i < length; i++ {
            bests[i] = max(bests[i-1], bests[i-2] + nums[i]);
        }
        return bests[length-1]; 
    }
}

func max(a,b int) int {
    if a>b {
        return a
    }
    return b
}
```
