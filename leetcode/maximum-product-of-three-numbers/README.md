# Maximum Product of Three Numbers

https://leetcode.com/problems/maximum-product-of-three-numbers/description/

## Summary

One approach to solving this problem involves traversing the array to identify the three maximum values and the two minimum values. Afterward, we compare the product of the three maximum values with the product of the maximum value and the two minimum values. This is a valuable step because if the two minimum values are negative, their multiplication will result in a positive value, underscoring the importance of tracking them.

## Code

```go
func maximumProduct(nums []int) int {
    max1 := math.MinInt
    max2 := math.MinInt
    max3 := math.MinInt

    min1 := math.MaxInt
    min2 := math.MaxInt

    for _, num := range nums {
        if num > max1 {
            max3 = max2
            max2 = max1
            max1 = num
        } else if num > max2 {
            max3 = max2
            max2 = num
        } else if num > max3 {
            max3 = num
        }

        if num < min1 {
            min2 = min1
            min1 = num
        } else if num < min2 {
            min2 = num
        }
        // fmt.Println("DEBUG: ", index, max1, max2, max3, min1, min2)
    }
    // fmt.Println("DEBUG: ", max1, max2, max3, min1, min2)
    return max(max1 * max2 * max3, max1 * min1 * min2)
}

func max(a,b int) int {
    if a > b {
        return a
    }
    return b
}
```

```rust
impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut max1 = std::i32::MIN;
        let mut max2 = std::i32::MIN;
        let mut max3 = std::i32::MIN;

        let mut min1 = std::i32::MAX;
        let mut min2 = std::i32::MAX;

        for num in nums {
            if num > max1 {
                max3 = max2;
                max2 = max1;
                max1 = num;
            } else if num > max2 {
                max3 = max2;
                max2 = num;
            } else if num > max3 {
                max3 = num;
            }

            if num < min1 {
                min2 = min1;
                min1 = num;
            } else if num < min2 {
                min2 = num;
            }
        }

        return std::cmp::max(max1 * max2 * max3, max1 * min1 * min2)
    }
}
```
