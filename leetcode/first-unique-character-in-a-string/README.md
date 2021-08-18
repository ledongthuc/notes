# First Unique Character in a String

https://leetcode.com/problems/first-unique-character-in-a-string/

## Summary

- Use hashmap with size 26. it will stores the character ASCII code counter.
- Loop through string `s`
	- Any character in `s` will increase counter at position `i` (which match with characeter's ascii) of hashmap
 - Loop through string `s` again, and return the first character which has counter = 1

## Code

```go
func firstUniqChar(s string) int {
    counter := make([]int, 26)
	for i := range s {
		counter[s[i] - 'a']++
	}
	for i := range s {
		if counter[s[i] - 'a'] == 1 {
			return i
		}
	}
	return -1
}
```

