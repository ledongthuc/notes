# Group Anagrams

https://leetcode.com/problems/group-anagrams

## Summary

 - Use hash map to store anagrams
 - With each item in input array, we sort it as key of hash map
 - Collect all anagrams from hash map

## Code

```go
func groupAnagrams(strs []string) [][]string {
    r := map[string][]string{}
    for _, str := range strs {
        k := SortString(str)
        r[k] = append(r[k], str)
    }
    
    result := make([][]string, 0, len(r))
    for _, arr := range r {
        result = append(result, arr)
    }
    return result
}

type sortRunes []rune

func (s sortRunes) Less(i, j int) bool {
    return s[i] < s[j]
}

func (s sortRunes) Swap(i, j int) {
    s[i], s[j] = s[j], s[i]
}

func (s sortRunes) Len() int {
    return len(s)
}

func SortString(s string) string {
    r := []rune(s)
    sort.Sort(sortRunes(r))
    return string(r)
}
```
