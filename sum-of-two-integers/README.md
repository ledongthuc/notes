```go
// https://en.wikipedia.org/wiki/Adder_%28electronics%29#Half_adder
func getSum(a int, b int) int {
    var carry int
    for b != 0 {
        carry = a & b
        a = a ^ b
        b = carry << 1  
    }
    return a;
}
```
