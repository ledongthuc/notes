# Merge Sorted Array

https://leetcode.com/problems/merge-sorted-array/

## Summary

 - Move nums1 to bottom, keep free front part for the result.

![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/merge-sorted-array/MergeSortedArray-push%20data%20back.png)

 - Loop through nums1 (bottom part) and nums2, compare and pick a smaller element to the front part of nums1.
 
 ![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/merge-sorted-array/MergeSortedArray-merge.png)


```go
func merge(nums1 []int, m int, nums2 []int, n int) {
	if n == 0 {
		return
	}
	if m == 0 {
	        copy(nums1, nums2)
		return
	}
	pushDataBack(nums1, m, n)
	for im, in, i := n, 0, 0; (im < m+n || in < n) && i < m+n; i++ {
		if (im < m+n && in >=  n) || (im < m+n && nums1[im] < nums2[in]) {
			nums1[i] = nums1[im]
			im++
		} else if in < n {
			nums1[i] = nums2[in]
			in++
		} else {
			break
		}
	}
}

func pushDataBack(nums []int, m, n int) {
	for i := m - 1; i >= 0; i-- {
		nums[i+n] = nums[i]
	}
}
```
