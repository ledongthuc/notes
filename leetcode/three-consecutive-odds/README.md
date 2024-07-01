# Three Consecutive Odds

https://leetcode.com/problems/three-consecutive-odds/description/

## Summary

So, basically, the logic to solve this problem is try to check item by item, if it's odd number, then we continue to check next following 2 items.
To improve, we can skip 1, 2 if the 1st and 2nd following items are even.

## Code

```go
func threeConsecutiveOdds(arr []int) bool {
    for i := 0; i < len(arr) - 2; {
        if arr[i + 2] % 2 == 0 {
            i += 3
            continue
        }
        if arr[i + 1] % 2 == 0 {
            i += 2
            continue
        }
        if arr[i] % 2 == 0 {
            i +=1
            continue
        }

        return true
    }
    return false
}
```
