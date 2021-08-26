# Reverse Bits

https://leetcode.com/problems/reverse-bits/

## Summary

 - Make a loop base on the size of input:
	 - Pop the right bit of input num (get right bit + shift right)
	 - Push the bit to right of new result (shift left + update right bit)
 - Finally, new result will be reverse of input

```
┌─┬─┬─┬─┬─┬─┬─┬─┐       
│ │ │ │ │ │ │ │ │─────┐ 
└─┴─┴─┴─┴─┴─┴─┴─┘     ▼ 
                     ┌─┐
                     │█│
                     └─┘
                      │ 
        ┌─┬─┬─┬─┐     │ 
        │ │ │ │ │◀────┘ 
        └─┴─┴─┴─┘       
``` 

## Code

```go
func reverseBits(num uint32) uint32 {
    var result uint32
    for i := 0; i < 32; i++ {
        result  = result << 1
        if num & 1 == 1{
            result++
        }
        num = num >> 1
    }
    return result
}
```

