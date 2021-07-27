# Implement Trie (Prefix Tree)

https://leetcode.com/problems/implement-trie-prefix-tree

## Summary

- Use tree to implement. Node's alias of a character of word. Words in a word presents by node's children.
- Each node has 2 important info:
	- Child nodes
	- Is it a word
- Insert: from root, check and create children nodes base on characters of the word, sequencely
- Search: from root, traversal to child nodes base on characters of the word.
	- If node of a character doesn't exist: word is not existed
	- If final node of final character isn't a word: word is not existed
- StartsWith: Same with Search, but doesn't check final node of final character. It's still valid if final node is not a full word

## Code

```go
type TrieNode struct {
    childs map[rune]*TrieNode
    isWord bool
}

type Trie struct {
    root *TrieNode
}


/** Initialize your data structure here. */
func Constructor() Trie {
    return Trie {
        root: &TrieNode{
            childs: map[rune]*TrieNode{},
            isWord: false,
        },
    }
}


/** Inserts a word into the trie. */
func (this *Trie) Insert(word string)  {
    node := this.root
    for _, char := range word {
        if _, existed := node.childs[char]; !existed {
            node.childs[char] = &TrieNode{
                childs: map[rune]*TrieNode{},
                isWord: false,
            }
        }
        node = node.childs[char]
    }
    node.isWord = true
}


/** Returns if the word is in the trie. */
func (this *Trie) Search(word string) bool {
    node := this.root
    for _, char := range word {
        if _, existed := node.childs[char]; !existed {
            return false
        }
        node = node.childs[char]
    }
    return node.isWord
}


/** Returns if there is any word in the trie that starts with the given prefix. */
func (this *Trie) StartsWith(prefix string) bool {
    node := this.root
    for _, char := range prefix {
        if _, existed := node.childs[char]; !existed {
            return false
        }
        node = node.childs[char]
    }
    return true
}


/**
 * Your Trie object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Insert(word);
 * param_2 := obj.Search(word);
 * param_3 := obj.StartsWith(prefix);
 */
```
