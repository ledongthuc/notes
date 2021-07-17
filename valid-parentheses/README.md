```go
func isValid(s string) bool {
    starts := map[string]struct{}{
        "(":struct{}{}, 
        "[":struct{}{}, 
        "{":struct{}{}, 
    }
    ends := map[string]struct{}{
        ")": struct{}{},
        "]": struct{}{},
        "}": struct{}{},
    }
    maps := map[string]string {
        ")":"(", 
        "]":"[", 
        "}":"{",
    }
    
    var stack StackString 
    for _, c := range s {
        _, ok := starts[string(c)]
        if ok {
            stack.Push(string(c))
        }
        _, ok = ends[string(c)]
        if ok {
            if stack.Length() == 0 || stack.Pop() != maps[string(c)] {
                return false
            }
        }
    }
    
    return stack.Length() == 0
}

type StackString struct {
    innerList []string
}

func (s *StackString) Length() int {
    return len(s.innerList)
}

func (s *StackString) Push(item string) {
    s.innerList = append(s.innerList, item)
}

func (s *StackString) Pop() string {
    v := s.innerList[len(s.innerList) - 1]
    s.innerList = s.innerList[0: len(s.innerList) -1]
    return v
}
```
