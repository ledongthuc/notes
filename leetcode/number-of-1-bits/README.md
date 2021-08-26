# Number of 1 Bits

https://leetcode.com/problems/number-of-1-bits/

## Summary

 - Make a loop base on the size of input:
    - Check and increase the counter if right bit is 1
    - Right shift

## Code

```go
func hammingWeight(num uint32) int {
    var count int
    for i:=0; i<32; i++ {
        if num & 1 == 1 {
            count++
        }
        num = num >> 1
    }
    return count
}
```

