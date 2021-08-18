# Making File Names Unique

https://leetcode.com/problems/making-file-names-unique

## Summary

- Use a hashmap of word and counter.
	- Every word in the hash will cache the last duplicated number
-  Each word from the parameter
	- if it's inexistent, then keep using that word and update this word to the hash table with counter=1
	- If it's existed, get counter and create the next unique name with the increased counter. Continue increasing the unique name until it's inexistent in the hash table. 

## Code

```go
type word string
type count int

func getFolderNames(names []string) []string {
    mapping := map[word]count{}
    for i, name := range names {
        uniqueName, c := getFolderName(name, mapping)
        mapping[word(uniqueName)]++
        mapping[word(name)] = c + 1
        names[i]= uniqueName
    }
    return names
}

func getFolderName(name string, mapping map[word]count) (string, count) {
    if _, existed := mapping[word(name)]; !existed {
        return name, count(0)
    }
    
    c := mapping[word(name)] - 1
    tmpName := ""
    for {
        c++
        tmpName = fmt.Sprintf("%s(%d)", name, c)
        
        if _, existed := mapping[word(tmpName)]; !existed  {
            return tmpName, c
        }
    }
}
```
