# Knight Dialer

https://leetcode.com/problems/knight-dialer/

## Summary

Let's start with something simple:
 - we need to find distict mobile numbers phone numbers of length `n`
 - Each valid number is created by knight jump of previous number

### Convert 2-dimensional pad to array

In order to easy to manage and calculation, we should convert the mobile phone 2-dimensional pad to array.
With array:
- Jumping to next right pad, the index +1,
- Jumping to next bottom pad, the index +3,


![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/knight-dialer/KnightDialer-pad2arr.png)

### Jumping boundary

As mobile phone pad layout, the knight can't jump of of the pad. So we need to validate valid jumping of knight on array to make sure it doesn't jump out of the pad

![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/knight-dialer/KnightDialer-boundary.png)

Let's check column jumping, maxixum a knight can jump left/right at ay position is:
 - Left jump: `-(current position % 3)` with `3` is number of pad on a row.
	 - index: 0 (pad 1) => left jump = 0
	 - index: 1 (pad 2) => left jump = -1
	 - index: 2 (pad 3) => left jump = -2
 - Right jump: `2 - (current position % 3)` with `2` is maximum a knight can jump without boundary and `3` is number of pad on a row.
	 - index: 0 (pad 1) => right jump = 2
	 - index: 1 (pad 2) => right jump = 1
	 - index: 2 (pad 3) => right jump = 3
 - Up jump: `max(-2, -int(currentPosition / 3))` with `3` is number of pad on a row, `-2` is maximum kinght can jump up.
	- index: 0 (pad 1) => up jump = 0
	- index: 3 (pad 4) => up jump = -1
	- index: 6 (pad 7) => up jump = -2
 - Down jump: `min(2, 3 - int(currentPosition / 3))` with `3` is number of pad on a row, `2` is maximum kinght can jump down.
	- index: 0 (pad 1) => down jump = 2
	- index: 3 (pad 4) => down jump = 1
	- index: 6 (pad 7) => down jump = 0

### Available jumps

Bases on boundary that kight can jump from current position, we can calculate available next jumps with conditions:
 - Knight only jump in range of boundary
 - Knight doesn't jump into index 9 or 11, the pad `*` and `#`
 - Knight doesn't jump verticle or horizontal: increasement of x or y are never 0
 - Knight doesn't jump diagonal: abs of increasement of x is never same with abs increasement of y

## Backtracking

Let's start with something simple:
 - we need to find distict mobile numbers phone numbers of length `n`
 - Each valid number is created by knight jump of previous number
it's backtracking pattern with starting is all valid number of the pad. We go through all cases and collect all possible valid mobile numbers to have result

![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/knight-dialer/KnightDialer-Backtrackking.png)

### Code

```go
const maxPad = 11
const positionIncreaseEachColumnJump = 1
const positionIncreaseEachRowJump = 3

func knightDialer(n int) int {
    total := 0
    for i:=0; i <= maxPad; i++ {
        if !isValidPosition(i) {
            continue
        }
        
        total += jump(n-1, i)
    }
    return total
}

// from current position, how many position knight can reach to
func jump(remainingStep int, currentPosition int) int {
    if remainingStep == 0 {
        if isValidPosition(currentPosition) {
            return 1
        }
        return 0
    }
    
    total := 0
    for _, newPosition := range availableJumps(currentPosition) {
        total += jump(remainingStep-1, newPosition)
    }
    return total
}

func availableJumps(currentPosition int) []int {
    minJumpIncreasementCol, maxJumpIncreasementCol, minJumpIncreasementRow, maxJumpIncreasementRow := getJumpIncreasementBoundary(currentPosition)
    availableJumps := make([]int, 0, 6)
    
    for colIncr := minJumpIncreasementCol; colIncr <= maxJumpIncreasementCol; colIncr++ {
        if colIncr == 0 {
            continue
        }
        
        for rowIncr := minJumpIncreasementRow; rowIncr <= maxJumpIncreasementRow; rowIncr ++ {
            if rowIncr == 0 || colIncr == rowIncr || colIncr + rowIncr == 0 {
                continue
            }
            
            newPosition := currentPosition + (positionIncreaseEachColumnJump * colIncr) + (positionIncreaseEachRowJump * rowIncr)
            if isValidPosition(newPosition) {
                availableJumps = append(availableJumps, newPosition)
            }            
        }
    }
    
    return availableJumps
}


func getJumpIncreasementBoundary(currentPosition int) (minJumpCol, maxJumpCol, minJumpRow, maxJumpRow int) {
    maxCol := 2

    minJumpCol = -(currentPosition % 3)
    maxJumpCol = maxCol - (currentPosition % 3)
    
    maxRow := 3
    minJumpRow = -int(currentPosition / 3)
    if minJumpRow < -2 {
      minJumpRow = -2
    }
    maxJumpRow = maxRow - int(currentPosition / 3)
    if maxJumpRow > 2 {
      maxJumpRow = 2
    }
    
    return minJumpCol, maxJumpCol, minJumpRow, maxJumpRow
}

// 9 = *
// 11 = #
func isValidPosition(position int) bool {
    if position < 0  || position == 9 || position > 10 {
        return false
    }
    return true
}
```

## Caching results

### Caching result

we will see a simple pattern:
In order to find number of distint mobile numbers of end at key pad `x2` with length `n`
It's we find number of mobile phone numbers of key pad `x1` with length `n-1` with conditions:
 - `x1` is valid key pad that can jump to `x2` in next steps
 - In order to jump to `x2`, have many `x1` can do it, so need to sum all of case can lead to `x1`
And because it's backtrack, with duplicated `key` and `n`, we can cache them to reuse later

![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/knight-dialer/KnightDialer-Backtracking%20cache.png)

### Caching available jumps

Another caching we can improve is available jumps of knight at any position.
We only need to pre-calculate them and reuse them in during problem solving.
The pad is fixed, so available jumps of knight won't change.

### Code

```go
const mod = 1e9 + 7 // Result's large. Must mod
const maxPad = 11   // Convert table pad to array 11 numbers

// Use to cache result from last processing bases on: pad number + remaining phone number length
type cacheKey struct {
    position int
    phoneNumberLength int
}
type cacheResult map[cacheKey]int

type cacheMoves [11][]int

func knightDialer(n int) int {
    total := 0
    cResult := cacheResult{}
    cMoves := getCacheMoves()
    
    for i := 0; i <= maxPad; i++ {
        if isValidPosition(i) {
            total += int(jump(n-1, i, cResult, cMoves) % mod)
        }
    }
    return int(total % mod)
}

// from current position, how many position knight can reach to
func jump(phoneNumberLength int, currentPosition int, cResult cacheResult, cMoves cacheMoves) int {
    // If end of number is invalid pad number (*, #), don't count
    if phoneNumberLength == 0 {
        if isValidPosition(currentPosition) {
            return 1
        }
        return 0
    }
    
    // Reuse cache value of pad number + length
    cKey := cacheKey{currentPosition, phoneNumberLength}
    if total, isExisted := cResult[cKey]; isExisted {
        return total
    }
    
    total := 0
    for _, newPosition := range cMoves[currentPosition] {
        total += int(jump(phoneNumberLength-1, newPosition, cResult, cMoves) % mod)
    }
    
    total = int(total % mod)
    cResult[cKey] = total
    return total
}

// from any number pad, we cache the their moving to reuse later
func getCacheMoves() cacheMoves {
    cMoves := cacheMoves{}
    for i := 0; i <= maxPad; i++ {
        if isValidPosition(i) {
            cMoves[i] = availableJumps(i)
        }
    }
    return cMoves
}

/*Convert table pad to array 11 numbers.
If we move to right on table pad, it's +1 in array number
If we move to bottom on table pad, it's +3 in array number*/
const positionIncreaseEachColumnJump = 1
const positionIncreaseEachRowJump = 3

// With any position, return new possitions that kight can jump to.
// except:
//  + Don't jump to * and #
//  + Don't jump to out of boundary
func availableJumps(currentPosition int) []int {
    minJumpIncreasementCol, maxJumpIncreasementCol, minJumpIncreasementRow, maxJumpIncreasementRow := getJumpIncreasementBoundary(currentPosition)
    availableJumps := make([]int, 0, 6)
    
    for colIncr := minJumpIncreasementCol; colIncr <= maxJumpIncreasementCol; colIncr++ {
        if colIncr == 0 {
            continue
        }
        
        for rowIncr := minJumpIncreasementRow; rowIncr <= maxJumpIncreasementRow; rowIncr ++ {
            if rowIncr == 0 || colIncr == 0 || colIncr == rowIncr || colIncr + rowIncr == 0 { // Knight is not a queen :)
                continue
            }
            
            newPosition := currentPosition + (positionIncreaseEachColumnJump * colIncr) + (positionIncreaseEachRowJump * rowIncr)
            if isValidPosition(newPosition) {
                availableJumps = append(availableJumps, newPosition)
            }            
        }
    }
    
    return availableJumps
}

/* Main idea is boundry of any position is it can't reach to smaller 0 and don't go to max of row/col */
func getJumpIncreasementBoundary(currentPosition int)(minJumpColOfCurrentPosition, 
                                                      maxJumpColOfCurrentPosition, 
                                                      minJumpRowOfCurrentPosition, 
                                                      maxJumpRowOfCurrentPosition int) {
        
    maxJumpCol := 2
    minJumpColOfCurrentPosition = -(currentPosition % 3)
    maxJumpColOfCurrentPosition = maxJumpCol - (currentPosition % 3)
    
    maxJumpRow := 3
    minJumpRowOfCurrentPosition = -int(currentPosition / 3)
    if minJumpRowOfCurrentPosition < -2 {
      minJumpRowOfCurrentPosition = -2
    }
    maxJumpRowOfCurrentPosition = maxJumpRow - int(currentPosition / 3)
    if maxJumpRowOfCurrentPosition > 2 {
      maxJumpRowOfCurrentPosition = 2
    }
    
    return
}

// 9 = *
// 11 = #
func isValidPosition(position int) bool {
    if position < 0  || position == 9 || position > 10 {
        return false
    }
    return true
}
```

## Reduce recursive

Last thing we can do is reducing the recursive in the solution.
We known about distint mobile numbers end with key pad `x` with lengh `n` is total of mobile phones number of any valid key pad `x1` can jump to `x` with length `n-1`.
So, we just need to go with n = range 1 -> n, and calculate total mobile phone of all valid key pad.
 - with pattern: cache(x, n) = Sum[ cache(x1, n-1) ] with x1 is all key pad can jump to `x`
Total distint mobile numbers are total result of all valid key 

### Code

```go
const mod = 1e9 + 7 // Result's large. Must mod
const maxPad = 11   // Convert table pad to array 11 numbers

// Use to cache result from last processing bases on: pad number + remaining phone number length
type cacheKey struct {
    position int
    phoneNumberLength int
}
type cacheResult map[cacheKey]int

type cacheMoves [11][]int

// recursive reduction
func knightDialer(n int) int {
    if n == 1 {
        return 10
    }
    
    total := 0
    cResult := cacheResult{}
    cMoves := getCacheMoves()
    
    
        
    for ni := 1; ni <= n; ni++ {
        for i := 0; i < maxPad; i++ { // 0 -> 9
            if !isValidPosition(i) {
                continue
            }
            if ni == 1 {
                cResult[cacheKey{i, ni}] = 1
                continue
            }

            totalOfMoves := 0
            for _, nextMove := range cMoves[i] {
                totalOfMoves = int((totalOfMoves + cResult[cacheKey{nextMove, ni - 1}]) % mod)
            }
            cResult[cacheKey{i, ni}] = totalOfMoves

            if ni == n {
                total = int((total + totalOfMoves) % mod)
            }
        }
    }
        
    return total
}

// from any number pad, we cache the their moving to reuse later
func getCacheMoves() cacheMoves {
    cMoves := cacheMoves{}
    for i := 0; i <= maxPad; i++ {
        if isValidPosition(i) {
            cMoves[i] = availableJumps(i)
        }
    }
    return cMoves
}

/*Convert table pad to array 11 numbers.
If we move to right on table pad, it's +1 in array number
If we move to bottom on table pad, it's +3 in array number*/
const positionIncreaseEachColumnJump = 1
const positionIncreaseEachRowJump = 3

// With any position, return new possitions that kight can jump to.
// except:
//  + Don't jump to * and #
//  + Don't jump to out of boundary
func availableJumps(currentPosition int) []int {
    minJumpIncreasementCol, maxJumpIncreasementCol, minJumpIncreasementRow, maxJumpIncreasementRow := getJumpIncreasementBoundary(currentPosition)
    availableJumps := make([]int, 0, 6)
    
    for colIncr := minJumpIncreasementCol; colIncr <= maxJumpIncreasementCol; colIncr++ {
        if colIncr == 0 {
            continue
        }
        
        for rowIncr := minJumpIncreasementRow; rowIncr <= maxJumpIncreasementRow; rowIncr ++ {
            if rowIncr == 0 || colIncr == 0 || colIncr == rowIncr || colIncr + rowIncr == 0 { // Knight is not a queen :)
                continue
            }
            
            newPosition := currentPosition + (positionIncreaseEachColumnJump * colIncr) + (positionIncreaseEachRowJump * rowIncr)
            if isValidPosition(newPosition) {
                availableJumps = append(availableJumps, newPosition)
            }            
        }
    }
    
    return availableJumps
}

/* Main idea is boundry of any position is it can't reach to smaller 0 and don't go to max of row/col */
func getJumpIncreasementBoundary(currentPosition int)(minJumpColOfCurrentPosition, 
                                                      maxJumpColOfCurrentPosition, 
                                                      minJumpRowOfCurrentPosition, 
                                                      maxJumpRowOfCurrentPosition int) {
        
    maxJumpCol := 2
    minJumpColOfCurrentPosition = -(currentPosition % 3)
    maxJumpColOfCurrentPosition = maxJumpCol - (currentPosition % 3)
    
    maxJumpRow := 3
    minJumpRowOfCurrentPosition = -int(currentPosition / 3)
    if minJumpRowOfCurrentPosition < -2 {
      minJumpRowOfCurrentPosition = -2
    }
    maxJumpRowOfCurrentPosition = maxJumpRow - int(currentPosition / 3)
    if maxJumpRowOfCurrentPosition > 2 {
      maxJumpRowOfCurrentPosition = 2
    }
    
    return
}

// 9 = *
// 11 = #
func isValidPosition(position int) bool {
    if position < 0  || position == 9 || position > 10 {
        return false
    }
    return true
}
```
