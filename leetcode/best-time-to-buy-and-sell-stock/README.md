
# Best Time to Buy and Sell Stock

https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

## Summary

We need to find the maximum profit between buying and selling on a range of dates.
Or technically, we compare any profit if we buy in a date and sell it later.
The final comparison will be the best profit we can have in range date.
We got the idea, but we have two approaches to solve this issue:
 - From chart view
 - From table accounting view

## Chart view

So take a look at the chart, it's familiar if we have experience in investment.

![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/best-time-to-buy-and-sell-stock/BestTimeToBuyAndSellStock-Chart.png)

From the chart, we have 2 things here:
 - Date from t(0) to t(n)
 - Stock values v(0) to v(n)

Classical, to calculate the profit, every investor will go date by date, from t(0) => t(n)
 - If we buy a stock on `t-1`, but on `t` the price's down. It means we don't have any profit
 - If we buy a stock on `t-1`, and on `t` the price's up, it means we have a profit

To keep the maximum stock profit, we need to buy at stock value is min and sell it at top stock value.

the solution here is we go through t(0) to t(n), cache the max profit and last min stock value
 - If stock on `t` is less than the last Min Stock Value, it means v(t) will be the last Min Stock Value. In this case, the stock's price is down and of course, we don't have any profit. We update the Min Stock Value to v(t) means we understand we should buy it at v(t), when it's cheaper.
 - If the stock value on `t` is greater than Min Stock Value, we will calculate profit from `v(t) - Last Min Stock Value`. We compare it with Max profit. If it's greater than, it mean we're better profit if we sell it on `v(t)`, but it's less than, we ignore it, and we shouldn't sell it on `t` if we want maximum final profit.
 - Finally, after finish (n) dates, we will know what's maximum profit if we buy/sell at the best timing from the cache of Max Profit.

![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/best-time-to-buy-and-sell-stock/BestTimeToBuyAndSellStock-Chart-line.png)

```go
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
```

## Table accounting view

Forget about charts and investment view. Let's take a look at the accounting view:
 - We find Maximum Profit every day (even sell or not sell anything)
 - To find Maximum Profit every day, we need to know what's last minimum stock value when we bought it before.

### Last minimum stock value of everyday

Every day, we need to know what is last minimum stock before when we buy the stock. Base on it, we can calculate daily profit.

To calculate it, we compare the minimum stock value of yesterday with today stock value. The smaller value will be the minimum stock value today.

```
minValue(t) = getMin( minValue(t-1) , value(t) )
```

### Maximum Profit everyday

Every day, the maximum profit is a comparison between the last Maximum Profit with the profit if today we sell the stock with the last minimum stock value.

```
MaxProfit(t) = getMax( MaxProfit(t-1), value(t) - minValue(t-1) )
```

So if we have range n dates, the MaxProfit at date (n) will be the MaxProfit if we buy/sell stock at the best timing during n dates.

### Code

```go
func maxProfit(prices []int) int {
    minAtTime := make([]int, len(prices), len(prices))
    maxProfitAtTime := make([]int, len(prices), len(prices))
    
    for i, price := range prices {
        if i == 0 {
            minAtTime[i] = price
            maxProfitAtTime[i] = 0 
            continue
        }
        minAtTime[i] = min(minAtTime[i-1], price)
        maxProfitAtTime[i] = max(maxProfitAtTime[i-1], price - minAtTime[i])
    }
    
    return maxProfitAtTime[len(prices) - 1]
}

func min(i, j int) int {
    if i < j {
        return i
    }
    return j
}

func max(i, j int) int {
    if i > j {
        return i
    }
    return j
}
```
