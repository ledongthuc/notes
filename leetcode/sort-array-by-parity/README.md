# Sort Array By Parity

https://leetcode.com/problems/sort-array-by-parity/

## Summary

 - Use 2 pointers:
	 - One starts from beginning of the array
	 - One starts from ending of the array
 - Loop until 2 pointer points to same element
	 - If beginning pointer is pointing to odd number, swap it to ending pointer. Then ending pointer walks back 1 item
	 - If beginning pointer is pointing to even number, skip it and walk forward 1 item
 - End of the loop, the array will be sort 2 parts, even and odd numbers

## Code

```go
func sortArrayByParity(nums []int) []int {
    left, right := 0, len(nums) - 1
    for {
        if left == right {
            break
        }
        
        if isOdd(nums[left]) {
            nums[left], nums[right] = nums[right], nums[left]
            right--
            continue
        }
        
        left++
    }
    return nums
}

func isEven(i int) bool {
    return i % 2 == 0
}

func isOdd(i int) bool {
    return i % 2 != 0
}
```
