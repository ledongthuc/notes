# Number of Islands

https://leetcode.com/problems/number-of-islands

## Summary

 - Start loop all cells of grid.
 - Found a water cell, skip.
 - Found a land cell:
	 - Number of islands ++
	 - Mark this cell is checked (we don't need to recheck again).
	 - Discover 4 directions (up/down/right/left) and continue to mark if their are land.
 - Found a checked/marked cell, skip.
 - Finally, we have total number of islands.

```go
func numIslands(grid [][]byte) int {
    if len(grid) == 0 {
		return 0
	}

	var numIslands int
        rows, cols := len(grid), len(grid[0])
	for i := 0; i < rows; i++ {
		for j := 0; j < cols; j++ {
			if grid[i][j] == 'x' {
				continue
			}

			if grid[i][j] == '1' {
				discoverIsland(grid, rows, cols, i, j)
				numIslands++
			} else {
				grid[i][j] = 'x'
			}
		}
	}

	return numIslands
}

func discoverIsland(g [][]byte, h, w, x, y int) {
    if (x < 0 || y < 0 || x >= h || y >= w) ||
    (g[x][y] == 'x' || g[x][y] == '0' ){
		return
	}

	g[x][y] = 'x'

	discoverIsland(g, h, w, x+1, y)
	discoverIsland(g, h, w, x, y+1)
	discoverIsland(g, h, w, x-1, y)
	discoverIsland(g, h, w, x, y-1)
}

```
