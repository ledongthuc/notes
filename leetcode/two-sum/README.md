# Two Sum

 - With each item in `nums`,  check if any existed number that's equal `target-num`. They are pair number of result

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

```rust
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (index, num) in nums.iter().enumerate() {
            let remaining = target - num;
            

            match map.get(&remaining) {
                Some(remainingIndex) => {
                    return vec![index as i32, *remainingIndex as i32];
                }
                _ => {
                    map.insert(num, index);
                }
            }
        }

        return Vec::new()
    }
}
```