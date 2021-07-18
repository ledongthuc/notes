# Excel Sheet Column Title

 - 1  -> 26: A -> Z | 26 / (26 + 1)
 - 27 -> 52: AA -> AZ | 52 / (26 + 1)
 - 53 -> 78: BA -> BZ | 78 / (26 + 1)

```go
func convertToTitle(n int) string {
    var result string
    for n > 0 {
        remain := n % 26
        if remain == 0 {
            result = "Z" + result
            n = (n / 26) -1
        } else {
            result = string(rune(remain - 1 + 65)) + result
            n = n / 26
        }
    }
    return result
}

/*

52 % 26 = 0
52 / 26 = 2

1  -> 26: A -> Z | 26 / (26 + 1)
27 -> 52: AA -> AZ | 52 / (26 + 1)
53 -> 78: BA -> BZ | 78 / (26 + 1)

*/
```
