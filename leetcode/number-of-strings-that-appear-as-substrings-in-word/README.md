# Number of Strings That Appear as Substrings in Word

https://leetcode.com/problems/number-of-strings-that-appear-as-substrings-in-word/

## Summary

 - Loop through pattern and check any pattern is existed in the word.

## Code

```go
func numOfStrings(patterns []string, word string) int {
    var count int
    for _, pattern := range patterns {
        if strings.Contains(word, pattern) {
            count++
        }
    }
    return count
}
```

