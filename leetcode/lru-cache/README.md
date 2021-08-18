# LRU Cache

https://leetcode.com/problems/lru-cache/

## Summary

- Use a double linked list to store and arrange the least recently used items
- Use a hash map between key and linked list items to quick access items by key
- When the client get the item by key, we check it from the hash table, move the item to the head of the linked list (recently used item)
- When the client put item by key:
	- If the key existed already, update its value and move the item to the head of the linked list (recently used item)
	- If it doesn't reach capacity; then add it to the hash table, add it to the head of the linked list (recently used item)
	- If it reaches capacity; then pop and remove the tail of the linked list, add to the hash table, add it to head of the linked list (recently used item)

## Code

```go
type LRUCache struct {
	Capacity int
	Length   int
	Table    map[int]*DoublyLinkedNode
	List     DoublyLinkedList
}

type DoublyLinkedList struct {
	Head *DoublyLinkedNode
	Tail *DoublyLinkedNode
}

type DoublyLinkedNode struct {
	Previous *DoublyLinkedNode
	Next     *DoublyLinkedNode

	Key   int
	Value int
}

func Constructor(capacity int) LRUCache {
	return LRUCache{
		Table:    make(map[int]*DoublyLinkedNode, capacity),
		List:     newEmptyDoublyLinkedList(),
		Capacity: capacity,
		Length:   0,
	}
}

func (this *LRUCache) Get(key int) int {
	node, isExisted := this.Table[key]
	if !isExisted || node == nil {
		return -1
	}
	this.List.MoveToHead(node)
	return node.Value
}

func (this *LRUCache) Put(key int, value int) {
	if node, isExisted := this.Table[key]; isExisted {
		node.Value = value
		this.List.MoveToHead(node)
		return
	}

	node := DoublyLinkedNode{
		Key:   key,
		Value: value,
	}
	if this.Length < this.Capacity {
		this.List.AddAtHead(&node)
		this.Table[key] = &node
		this.Length++
	} else {
		removed := this.List.PopAtTail()
		delete(this.Table, removed.Key)
		this.List.AddAtHead(&node)
		this.Table[key] = &node		
	}

}

func newEmptyDoublyLinkedList() DoublyLinkedList {
	list := DoublyLinkedList{
		Head: &DoublyLinkedNode{},
		Tail: &DoublyLinkedNode{},
	}
	list.Head.Next = list.Tail
	list.Tail.Previous = list.Head
	return list
}

func (l *DoublyLinkedList) AddAtHead(node *DoublyLinkedNode) {
	oldAfterNext := l.Head.Next
	l.Head.Next.Previous = node
	l.Head.Next = node

	node.Previous = l.Head
	node.Next = oldAfterNext
}

func (l *DoublyLinkedList) PopAtTail() *DoublyLinkedNode {
	node := l.Tail.Previous
	l.Remove(node)
	return node
}

func (l *DoublyLinkedList) Remove(node *DoublyLinkedNode) {
	node.Previous.Next = node.Next
	node.Next.Previous = node.Previous
	node.Previous = nil
	node.Next = nil
}

func (l *DoublyLinkedList) MoveToHead(node *DoublyLinkedNode) {
	l.Remove(node)
	l.AddAtHead(node)
}

```
