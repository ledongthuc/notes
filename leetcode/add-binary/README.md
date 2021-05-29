# Add Binary

https://leetcode.com/problems/add-binary/

## Summary

Adding binary units from right to left, sequencely
	 - 0 + 0 = 0
	 - 0  + 1 = 1 + 0 = 1
	 - 1 + 1 = 10, update 0 as unit result and carried 1 to next unit
Continue until end of strings

```go
func addBinary(a string, b string) string {
    var result string
    remaining := 0
    var cA, cB int
    var s int
    for {
        if len(a) == 0 && len(b) == 0 && remaining == 0 {
            return result
        }
        cA, a = GetP(a)
        cB, b = GetP(b)
        s, remaining = Add(cA, cB, remaining)
        if s == 0 {
            result = "0" + result
        } else {
            result = "1" + result
        }
    }
}

func GetP(s string) (int, string) {
    l := len(s)
    if l == 0 {
        return 0, s
    }
    if s[l - 1] == '1' {
        return 1, s[:l-1]
    }
    return 0, s[:l-1]
}

func Add(a, b, r int) (int, int) {
    s := a + b + r
    r = 0
    if s > 1 {
        r = 1
        if s == 2 {
            s = 0
        }
        if s == 3 {
            s = 1
        }
    }
    return s, r
}
```
