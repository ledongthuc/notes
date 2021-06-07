# Shuffle an Array

https://leetcode.com/problems/shuffle-an-array/

## Summary

- Use Fisher–Yates to suffer array (https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle)

- Generate pseudorandom Xorshift star (https://en.wikipedia.org/wiki/Xorshift#xorshift*)

```go
type Solution struct {
    state uint64
    originalArray []int
}


func Constructor(nums []int) Solution {
    return Solution{
        state: generateSeeding(uint64(time.Now().UnixNano())),
        originalArray: nums,
    }
}


/** Resets the array to its original configuration and return it. */
func (this *Solution) Reset() []int {
    return this.originalArray
}


/** Returns a random shuffling of the array. */
func (this *Solution) Shuffle() []int {
    l := len(this.originalArray)
    
    newArr := make([]int, len(this.originalArray))
    copy(newArr, this.originalArray)
    
    for i := l - 1; i > 0; i-- {
        this.state = generateSeeding(this.state)
        swapIndex := this.state % uint64(i+1)
        newArr[i], newArr[swapIndex] = newArr[swapIndex], newArr[i]
    }
    
    return newArr
}

// Xorshift star
func generateSeeding(state uint64) uint64 {
    newState := state
    newState ^= newState << 12
	newState ^= newState >> 25
	newState ^= newState << 27
    return newState;
}
```
