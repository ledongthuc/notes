# Majority Element

https://leetcode.com/problems/majority-element/

## Summary

 - Use a counter to check major number.
 - If a checking number appears in next number, counter + 1
 - If an other numbers appear in next number, counter - 1
 - if counter = 0, change checking number to another number.

## Code

```go
/* func majorityElement(nums []int) int {
    n := len(nums) / 2
    m := make(map[int]int)
    for _, num := range nums {
        m[num] = m[num] + 1
        if m[num] > n {
            return num
        }
    }
    return -1
} */

func majorityElement(nums []int) int {
    var maybeMajor, count int
    for _, num := range nums {
        if 0 == count {
            maybeMajor = num
            count++
        } else if maybeMajor == num {
            count++
        } else {
            count --
        }
    }
    return maybeMajor
}
```
