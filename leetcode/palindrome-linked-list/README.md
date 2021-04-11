# Palindrome Linked List

Given the head of a singly linked list, return true if it is a palindrome.
Constraints:
 - The number of nodes in the list is in the range [1, 105].
 - 0 <= Node.val <= 9

## Summary

We split the problem to 3 smaller problems:
 - Find middle point of linked list.
 - Reverse 2nd part of linked list.
 - Compare 1st part and reversed 2nd part of linked list.

## Find middle point of linked list

 - Use 2 pointers;
 - First pointer jumps 2 element, Second pointer jump 1 element.
 - Result of this step, we can split the linked list to 2 parts.

![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/palindrome-linked-list/palindrome_linked_list-Detect%20middle.png)
- Complexity: O(n)
- Space: O(2)

## Reverse half part of linked list

Reverse half part of linked list
 - Use 3 pointers
 - First pointer points to current processing element.
 - Second pointer point to the next item of element that's pointed by first pointer.
 - Third pointer use to point to next item of element that's pointed by second pointer, temporary, during swapping between elements.
 - Swap actions:
	 - Third pointer point to next element of element that's pointed by second pointer.
	 - First pointer element remove the link.
	 - Second pointer element update next link to element that's pointed by first pointer.
	 - First pointer points to second pointer element, second pointer points to third pointer element.
	 - Release third pointer.
- Swap until end of the parse, we have a reversed half part linked list

![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/palindrome-linked-list/palindrome_linked_list-Reverse%202nd%20part.png)
 - Complexity: O(n/2)
 - Space: O(3)

## Compare 2 parts

 - Compare reversed part and remaining part of linked list
 - Use two pointers.
 - Each pointer keep a current checking item of two parts.
 - Loop through item and compare their value.
 - If they of the same value, then it's a palindrome linked list.
![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/palindrome-linked-list/palindrome_linked_list-Compare%202%20linked%20list.png)
## Total
 - Complexity: O(2n)
 - Space: O(7)
