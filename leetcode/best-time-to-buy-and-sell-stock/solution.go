package leetcode

import "math"

/*
- Loop through price per dates
- Check and lock min price if checking date's smaller than current min-price
- If price of date is not smaller min-price, calculate profit and use it as max profit if it's larger
*/

func maxProfit(prices []int) int {
	minPrice := int(math.MaxInt32)
	maxProfit := 0
	for _, value := range prices {
		if minPrice > value {
			minPrice = value
		} else {
			profit := value - minPrice
			if profit > maxProfit {
				maxProfit = profit
			}
		}
	}
	return maxProfit
}
