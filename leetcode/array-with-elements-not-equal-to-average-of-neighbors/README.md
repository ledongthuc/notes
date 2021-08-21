# Check If String Is a Prefix of Array

https://leetcode.com/problems/array-with-elements-not-equal-to-average-of-neighbors/

## Summary

 - Go through item in array `nums` except first and last item.
	 - If the item is equal to the average of its neighbors, then swap it with next item
	 - Continue the looping through all item until no item is equal to the average of its neighbors

## Code

```go
func rearrangeArray(nums []int) []int {
    isContinue := true
    for isContinue {
        isContinue = false
        for i:=1; i< len(nums) -1; i++ {
            if (2 * nums[i] == nums[i-1] + nums[i+1]) {
                isContinue = true
                nums[i], nums[i+1] = nums[i+1], nums[i]
            }
        }
    }
    return nums;
}
```

