# Check If String Is a Prefix of Array

https://leetcode.com/problems/check-if-string-is-a-prefix-of-array/

## Code

```go
func isPrefixString(s string, words []string) bool {
    if len(words) == 0 && len(s) > 0 {
        return false
    }
    if len(s) < len(words[0]) {
        return false
    }
    if len(s) == len(words[0]) {
        return strings.HasPrefix(words[0], s)
    }
    if !strings.HasPrefix(s, words[0]) {
        return false
    }
    return isPrefixString(s[len(words[0]):], words[1:])
}
```

