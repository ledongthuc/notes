# Permutations

https://leetcode.com/problems/permutations/

## Summary
 - Based on backtracking (https://en.wikipedia.org/wiki/Backtracking)
 - Two solutions:
	 - Build tree then collect results.
	 - Swap elements.

## Tree
- Use elements as a root node of a tree, remaining elements are building the branches and leafs.
- With each remaining elements of a tree, attach it to root element to have branches.
- Each branches continue to use remaining elements (after remove root and first branch) to build until out of remaining elements.
 - Collect all leafs of the trees as result of permutations.

![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/permutations/Permutations-build%20tree%20%28step%29.png)
![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/permutations/Permutations-build%20tree%20%28full%29.png)

## Swap elements
 - Swap first element with remaining elements.
 - Lock first element and continue swapping remaining parts.
 - Continue swapping until we don't have remianing elements.
 - Collect all cases as result of permutations.

![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/permutations/Permutations-swap.png)
