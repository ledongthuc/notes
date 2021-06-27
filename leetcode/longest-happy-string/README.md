# Longest Happy String

https://leetcode.com/problems/longest-happy-string/

## Summary

 - Use priority queue to storing 'a', 'b', 'c' and their remaining counter as priority
	 - Highest priority means highest counter of 'a', 'b' or 'c'
 - Loop and pop from priority queue until the queue is empty
	 - Pop item is highest priority, we try to use this character first.
	 - If pop item's value is not same value with 2 previous item of result, then use it to build result; decrease counter and push back to the queue if counter is not 0
	 - If pop item's value is same with 2 previous item of result, then pop again and use 2nd highest priority item to build the result; decrease counter and push back 1st + 2nd item to the queue if counters are not 0

## Code

```go
type Item struct {
	value    rune
	priority int
	index    int
}

type PriorityQueue []*Item

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].priority > pq[j].priority
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *PriorityQueue) Push(x interface{}) {
	n := len(*pq)
	item := x.(*Item)
	item.index = n
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil
	item.index = -1
	*pq = old[0 : n-1]
	return item
}

func longestDiverseString(a int, b int, c int) string {
    raw := []*Item{
        &Item { value: rune('a'), priority: a },
        &Item { value: rune('b'), priority: b },
        &Item { value: rune('c'), priority: c },
    }
    pq := make(PriorityQueue, 0, a+b+c)
    for _, item := range raw {
        if item.priority > 0 {
            heap.Push(&pq, item)
        }
    }
    
    var previous1, previous2 rune
    var result strings.Builder

    for len(pq) > 0 {
        updatedItem := heap.Pop(&pq).(*Item)
        
        // If highest priority same with 2 previous item, use 2nd highest piority item as updated item
        if updatedItem.value == previous1 && updatedItem.value == previous2 {
            if len(pq) == 0 {
                break
            }
            
            secondItem := heap.Pop(&pq).(*Item)
            heap.Push(&pq, updatedItem)
            
            updatedItem = secondItem
        }
        
        result.WriteRune(updatedItem.value)
        updatedItem.priority--
        if updatedItem.priority > 0 {
            heap.Push(&pq, updatedItem)
        }

        previous2 = previous1
        previous1 = updatedItem.value
    }
    return result.String()
}
```
