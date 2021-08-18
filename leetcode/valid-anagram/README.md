# 1. Valid Anagram

https://leetcode.com/problems/valid-anagram

## 1.1. Summary

- Use hashmap with size 26. it will stores the character ASCII code counter.
- Loop through string `s1` and `s2`
	- Any character in `s1` will increase counter at position `i` (which match with characeter's ascii) of hashmap
	- Any character in `s2` will decrease counter at position `i` (which match with characeter's ascii) of hashmap
- `s1` and `s2` is an anagram if all hashmap counter is 0. Any character's in s1 will be in s2.

## 1.2. Code

```go
func isAnagram(s string, t string) bool {
    if len(s) != len(t) {
        return false
    }
    
    counter := make([]int, 26)
    
    for i, _ := range s {
        counter[s[i] - 'a']++
        counter[t[i] - 'a']--
    }
    
    for i, _ := range counter {
        if counter[i] != 0 {
            return false
        }
    }
    
    return true
}
```

