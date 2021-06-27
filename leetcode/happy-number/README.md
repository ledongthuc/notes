# Happy Number

https://leetcode.com/problems/happy-number/

## Summary

 - Split number to parts by
	 - Unit = number % 10
	 - Next digit = number / 10
 - Plus square of splitted numbers
	 - If result's 1 then number is happy
	 - If result's not 1 and never existed in previous results, then continue
	 - If result's not 1 and existed in previous results, then number is not happy

## Code

```go
func isHappy(n int) bool {
    arr := splitNumber(n)
    cache := map[int]struct{}{}
    
    for {
        if n = sumOfSquare(arr); n == 1 {
            return true
        }
        
        _, existed := cache[n]
        if existed {
            return false
        }
        
        cache[n] = struct{}{}
        arr = splitNumber(n)
    }
}

func sumOfSquare(arr []int) int {
    n := 0
    for _, digit := range arr {
        n += digit * digit 
    }
    return n
}

func splitNumber(n int) []int {
    if n < 10 {
        return []int{n}
    }
    
    result := []int{} 
    for {
        var unit int
        n, unit = getUnitNumber(n)
        result = append(result, unit)
        if n == 0 {
            break
        }
    }
    
    return result
}

func getUnitNumber(n int) (left, right int) {
    if n >= 10 {
        return n / 10, n % 10
    }
    return 0, n
}
```
