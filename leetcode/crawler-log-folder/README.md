# Crawler Log Folder

https://leetcode.com/problems/crawler-log-folder/description/

## Summary

Just a "if-else" follows the requirements

## Code

```go
func minOperations(logs []string) int {
    backCount := 0
    for _, log := range logs {
        switch {
            case log == "../" && backCount == 0:
                continue
            case log == "../":
                backCount--
            case log == "./":
                continue
            default:
                backCount++
        }
    }
    return backCount
}
```
