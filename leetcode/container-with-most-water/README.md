# Container With Most Water

https://leetcode.com/problems/container-with-most-water/

## Summary

 - Start to check from maximum width container
	 - Sequence, check left and right, move the smaller one to center if any higher verticle lines to continue to check max area

## Code

```go
func maxArea(height []int) int {
    maxArea := 0
    i, j := 0, len(height) - 1
    
    for i < j {
        smaller := min(height[i], height[j])
        maxArea = max(maxArea, smaller * (j -i))
        for i < j {
            if height[i] < height[j] {
                i++
                if height[i] > height[i-1] {
                    break
                }
            } else {
                j--
                if height[j] > height[j+1] {
                    break
                }
            }
        }
    }
    return maxArea
}

func max(i1, i2 int) int {
    if i1 > i2 {
        return i1
    }
    return i2
}

func min(i1, i2 int) int {
     if i1 < i2 {
        return i1
    }
    return i2   
}
```

