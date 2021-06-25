# Toeplitz Matrix

https://leetcode.com/problems/toeplitz-matrix

## First approach

 - Go through items of first rows and first column
 - With each item, check next diagonal items [ (+1,+1), (+2, +2), ... ] are match with it. If any item is not match with first item, the matrix is not toeplitz

![](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/toeplitz-matrix/1_col.png)
![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/toeplitz-matrix/1_row.png)

## Code

```go
func isToeplitzMatrix(matrix [][]int) bool {
    rowL := len(matrix)
    colL := len(matrix[0])
    
    for i := 0; i < colL; i++ {
        if !isValidDiagonalLine(matrix, 0, i, rowL-1, colL-1) {
           return false 
        }
    }

    for i := 0; i < rowL; i++ {
        if !isValidDiagonalLine(matrix, i, 0, rowL-1, colL-1) {
           return false 
        }
    }
    
    return true
}

func isValidDiagonalLine(matrix [][]int, row, col, maxRow, maxCol int) bool {
    v := matrix[row][col]
    for {
        if row > maxRow || col > maxCol {
            return true
        }
        if matrix[row][col] != v {
            return false
        }
        row++
        col++
    }
} */
```

## Second approach

 - Go through items of matrix, one by one from start to end.
 - With each item, it should match with previous diagonal item (-1,-1)

## Code

```go
func isToeplitzMatrix(matrix [][]int) bool {
    rowL := len(matrix)
    colL := len(matrix[0])
    for row := 0; row < rowL; row++ {
        for col := 0; col < colL; col++ {
            if row > 0 && col > 0 && matrix[row-1][col-1] != matrix[row][col] {
                return false
            }
        }
    }

    return true
}
```
