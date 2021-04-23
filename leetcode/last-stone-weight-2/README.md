# Last stone weight II

https://leetcode.com/problems/last-stone-weight-ii/

## Summary

Basically, problem requires us to try mash stone together to have last smallest remaining. So, the best scenarios is that the remaining is zero, it means each pair stones will be same weight, or their remaining of pair stone will have same weight.
It means:
```
StoneL1 + StoneL2 + ... + StoneL(n) = StoneR1 + StoneR2 + ... + StoneR(n)
or we can write
[StoneL1 + StoneL2 + ... + StoneL(n)] - [StoneR1 + StoneR2 + ... + StoneR(n)] = 0
```

![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/last-stone-weight-2/LastStoneWeight2-Best%20scenario.png)
But if it's not best scenario, then we need to find smallest (x) from, it will be:
![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/last-stone-weight-2/LastStoneWeight2-Normal%20scenario.png)

## Math thinking

In order to find x, we will have:
```
[StoneL1 + StoneL2 + ... + StoneL(n)] - [StoneR1 + StoneR2 + ... + StoneR(n)] = x
<=> [StoneL1 + StoneL2 + ... + StoneL(n)] = x + [StoneR1 + StoneR2 + ... + StoneR(n)]
```
Recheck again, we have another equation to calculate total weight stone we have:
```
[StoneL1 + StoneL2 + ... + StoneL(n)] + [StoneR1 + StoneR2 + ... + StoneR(n)] = Sum
<=> x + [StoneR1 + StoneR2 + ... + StoneR(n)] + [StoneR1 + StoneR2 + ... + StoneR(n)] = sum
<=> x = Sum  - 2 * [StoneR1 + StoneR2 + ... + StoneR(n)]
```
From here, we can understand, if we want to find smallest x, we find largest `[StoneR1 + StoneR2 + ... + StoneR(n)]` with condition:
```
Sum  - 2 * [StoneR1 + StoneR2 + ... + StoneR(n)] >= 0
<=> Sum/2 - [StoneR1 + StoneR2 + ... + StoneR(n)] >= 0
<=> Sum/2 >= [StoneR1 + StoneR2 + ... + StoneR(n)]
```
It means we will find largest `[StoneR1 + StoneR2 + ... + StoneR(n)]` that is less than `1/2 of Sum` total weight of stones

## Image thinking

Because Total left = Total right + remaining (if right's smaller than left)
Then, if we want find x, it mean we need to find total Right
And if we want to find smallest x, it means we need to find biggest total Right we can from available stones
![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/last-stone-weight-2/LastStoneWeight2-Find%20equation.png)

## Combinations

Next question is how we can find maximum stones that's less than `1/2 of sum weight of total stone`
Easiest solution is try all stone combinations that have weight is less than 1/2 of  sume, and get the most heaviest combinations.
![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/last-stone-weight-2/LastStoneWeight2-combination.png)
Let's re-thinking a little bit about the remaining of a stone checking.
Let's check a stone1 with 1kg, if we know any combination y = stone2 + stone3 + stone4 that maximum remaining of stone 1, we can confirm the combination is available to use.
![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/last-stone-weight-2/LastStoneWeight2-remaining%201.png)
The problem is we don't know any combination at beginning, so, we need to build the list caching of stone + their combination step by step:
![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/last-stone-weight-2/LastStoneWeight2-remaining%202.png)The maximum we have in the caching, will be maximum stones that we can combine to.
Finally, apply `x = Sum  - 2 * [StoneR1 + StoneR2 + ... + StoneR(n)]` to find smallest stone x

## Code

```go
func lastStoneWeightII(stones []int) int {
    totalStonesWeight := getTotalStonesWeight(stones)
    halfOfTotalStonesWeight := int(totalStonesWeight/2)
    cachedCombinations := map[int]struct{}{}
    maxCombination := 0
    
    for _, stone := range stones {
        if stone > halfOfTotalStonesWeight {
            continue
        }
        newCombinations := map[int]struct{}{stone: struct{}{}}
        maxCombination = max(stone, maxCombination)
        
        for cachedCombination := range cachedCombinations {
            newCombination := stone + cachedCombination
            if newCombination > halfOfTotalStonesWeight {
                continue
            }
            
            newCombinations[newCombination] = struct{}{}
            maxCombination = max(newCombination, maxCombination)
        }
        
        fillCacheToCache(newCombinations, cachedCombinations)
    }
    
    return totalStonesWeight - 2 * (maxCombination)
}

func getTotalStonesWeight(stones []int) (sum int) {
    for _, stone := range stones {
        sum += stone
    }
    return sum
}

func fillCacheToCache(from, to map[int]struct{}) {
    for item := range from {
        to[item] = struct{}{}
    }
}

func max(i, j int) int {
    if i > j {
        return i
    }
    return j
}
```
