# Climbing Stairs

 - Fibonanci detected
 - N = 3, ways = 1 + 2
 - N = 4, ways = 3 + 2
 - N = 5, ways = 4 + 3
 - etc

```go
func climbStairs(n int) int {
	if n <= 2 {
		return n
	}

	n1 := 1
	n2 := 2
	for i := 2; i < n; i++ {
		n1, n2 = n2, n1+n2
	}

	return n2
}
```
