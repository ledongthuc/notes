# Determine Color of a Chessboard Square

https://leetcode.com/problems/determine-color-of-a-chessboard-square/

## Summary

Let's convert the x-axis from characters to number to easy to check

![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/determine-color-of-a-chessboard-square/chess_board.png)

We can easy to see the pattern is:
 - x + y % 2 == 0, the cell's black
 - x + y % 2 == 0, the cell's white
So we easy to find the cell's colour.

```go
return ((coordinates[0] - 96) + (coordinates[1] - 48)) % 2 != 0
```
p/s:
 - 'a' character ASCII is 97
 - '1' character ASCII is 49
