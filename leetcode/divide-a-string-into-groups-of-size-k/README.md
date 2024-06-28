# Divide a String Into Groups of Size k

https://leetcode.com/problems/divide-a-string-into-groups-of-size-k/description/

## Summary

Loop through the input string character by character, and stack them up until the stack reaches a length of k.

Each time the stack reaches a length of k, we put it into the result and then reset the stack.

Finally, if the stack still contains values and hasn't reached a length of k, we fill it with padding characters and use it as the final result.

## Code

```go
func divideString(s string, k int, fill byte) []string {
    result  := make([]string, 0, len(s) / k)
    i := 0
    var strResult strings.Builder
    for _, r := range s {
        strResult.WriteRune(r)
        i++
        if(i == k) {
            i = 0
            result = append(result, strResult.String())
            strResult.Reset()
        }
    }

    remainingLen := strResult.Len()
    if remainingLen > 0 {
        for i:=0;i< k - remainingLen; i++ {
            strResult.WriteByte(fill)
        }
        result = append(result, strResult.String())
    }
    return result
}
```
