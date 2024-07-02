# Intersection of Two Arrays II

## Sumarry

Go through 1st array, and store the frequency in an array. Recheck 2nd array, if the same item in frequency is available, we count it to result and decrease frequency.

## Code

```go
func intersect(nums1 []int, nums2 []int) []int {
    frequencyArr := make([]int, 1001)
    result := []int{}
    for _, item := range nums1 {
        frequencyArr[item]++
    }
    for _, item := range nums2 {
        if frequencyArr[item] > 0 {
            result = append(result, item)
        }
        frequencyArr[item]--
    }
    return result
}

```

```rust
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut frequency_arr = vec![0; 1001];

        for item in nums1 {
            frequency_arr[item as usize] += 1;
        }

        let mut result = Vec::with_capacity(10);
        for item in nums2 {
            if frequency_arr[item as usize] > 0 {
                result.push(item)
            }
            frequency_arr[item as usize] -= 1;
        }

        return result;
    }
}
```

```zig
fn intersect(nums1: []const i32, nums2: []const i32) []i32 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer gpa.deinit();

    const allocator = gpa.allocator();

    var frequency_arr = try allocator.alloc(i32, 1001);
    defer allocator.free(frequency_arr);
    std.mem.set(frequency_arr[0..], 0);

    for (nums1) |item| {
        frequency_arr[item] += 1;
    }

    var result = try allocator.alloc(i32, 0);
    defer allocator.free(result);

    for (nums2) |item| {
        if (frequency_arr[item] > 0) {
            try result.append(item);
        }
        frequency_arr[item] -= 1;
    }

    return result;
}
```
