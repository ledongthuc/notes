```go
func generateParenthesis(n int) []string {
    if n == 1 {
        return  []string{"()"}
    }
    
    m := map[string]struct{}{}
    for leftN := 1; leftN < n; leftN++ {
        leftResults := generateParenthesis(leftN)
        
        rightN := n - leftN
        rightResults := generateParenthesis(rightN)
        
        for _, leftResult := range leftResults {
            for _, rightResult := range rightResults {
                m[leftResult+rightResult] = struct{}{}
            }
        }
    }
    
    for _, subResult := range generateParenthesis(n-1) {
        m["(" + subResult + ")"] = struct{}{}
        
    }
    
    result := []string{}
    for r := range m {
        result = append(result, r)
    }
    
    return result
}
```
