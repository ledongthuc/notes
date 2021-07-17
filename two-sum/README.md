```go
func twoSum(nums []int, target int) []int {
    m := make(map[int]int)
    for index, value := range nums {
        m[value] = index
    }
    for index, value := range nums {
        remaining := target - value
        if index2, ok := m[remaining]; ok {
            if index == index2 {
                continue
            }
            return []int{index, index2}
        }
    }
    
    return []int{0, 0}
}
```
