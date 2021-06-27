# The k-th Lexicographical String of All Happy Strings of Length n

https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/

## Summary
  - Use backtrack to build a tree with available option leaves is 'a', 'b' and 'c'
  - When building nodes of the tree, the option same with previous node will be skipped
  -  Result is sorted of leaves and get item at k

## Code

```go
var options = []string{"a", "b", "c"}

func getHappyString(n int, k int) string {
    result := buildResult(n, "", "")
    sort.Strings(result)
    if k > len(result) {
        return ""
    }
    return result[k-1]
}

func buildResult(n int, s string, previous string) []string {
    if n == 0 {        
        return []string{s}
    }
    
    result := []string{}
    for _, option := range options {
        if previous == option {
            continue
        }
        result = append(result, buildResult(n-1, s+option, option)...)
    }
        
    return result
}
```
