# Best Time to Buy and Sell Stock

https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/

## Summary

The solution of this problem is inherit from https://github.com/ledongthuc/notes/tree/master/leetcode/best-time-to-buy-and-sell-stock
It means we will go through date by date, find the maximum profit we can have in this date.
But first, let's check cases that we have on random date:

## Starting point

Nothing special here.
As we can see, we can't buy and sell to any profit from same date.
So t(n) with n = 0 always has profit is 0
```
maxProfit(0) = 0
```

![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/best-time-to-buy-and-sell-stock-with-cooldown/BestTimeToBuyAndSellStockWithCooldown-Chart-0.png)
## Cooldown date

If a date is cooldown date, it means we can't buy anything, and of couse we can't sell anything too (because have just sell yesterday).
So, max profit of cooldown date is always same profit of yesterday
```
maxProfit(t) = maxProfit(t-1)
```

![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/best-time-to-buy-and-sell-stock-with-cooldown/BestTimeToBuyAndSellStockWithCooldown-Chart-cooldown.png)
## Selling date

If we decide to sell on date (t), the profit will be the gain from buying price to price today (t)
The buying price is the min price from beginning to (t).

![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/best-time-to-buy-and-sell-stock-with-cooldown/BestTimeToBuyAndSellStockWithCooldown-Chart-sell.png)
But let's check an special case that we buy-sell many times, we will have cooldown date between buy-sell cycle

![enter image description here](https://raw.githubusercontent.com/ledongthuc/notes/master/leetcode/best-time-to-buy-and-sell-stock-with-cooldown/BestTimeToBuyAndSellStockWithCooldown-Chart-sell-with-cooldown.png)
So with this special case, we can see max profit of date (t) can be bigger if we can split the deal to multiple buy-sell.
Then max profit in this case should be:
```
MaxProfit(t) = getMax(price(t) - min, price(t) - price(x) + MaxProtfit(x-2))
| with x = range 2 -> t-1
```
With (x) is the the date in range from 0 to (t-1) that has best profit.

## Solution

Finally, to solve problem, we calculate the maximum profit of latest date.
To calculate maximum profit of latest date, we calculate maximum profit of previous dates.
To calculate maximum profit of a date (t), it's a decision making to get best profit between:
 - Sell stock on (t-1) and keep (t) as cooldown date
 - Sell stock today with buying date is (x) and sum with profit from (x-2). With (x) is the date that give best profit.

```
MaxProfit(t) = getMax( MaxProfit(t-1), getMax(price(t) - min, price(t) - price(x) + MaxProtfit(x-2)))
| with x = range 2 -> t-1
```

With (x) is the the date in range from 0 to (t-1) that has best profit.


**Optimize**

With each date (t), we need have a nested loop from 2 to (t-1) to find x, that's waste.
So let see the part with (x)

```
Profit(t) if cooldown on (x-1) = price(t) - price(x) + MaxProtfit(x-2)
<=>
Profit(t) if cooldown on (x-1) = price(t) - ( price(x) - MaxProtfit(x-2) )
```

So we have a form `price(x) - MaxProtfit(x-2)` , we called it's `C`

```
Profit(t) if cooldown on (x-1) = price(t) - C(x)
| with C(x) = price(x) - MaxProtfit(x-2)
| with x range 2 => t-1
```

In order to keep `Profit(t) if coolddown on (x-1)` is maximum value, with `price(t)` is fixed,
then `C` must be minimum value it can be.

So, we can calculate and cache the `MinC(x)` by pickup minimum value of:
 - MinC(x-1)
 - price(x) - MaxProtfit(x-2)

Then, we cache `MinC` everyday, we have

```
MaxProfit(t) if cooldown on (x-1) = price(t) - MinC(t-1)
| with MinC(t-1) = getMin( MinC(t-2), price(t-1) - MaxProtfit(t-3) )
```

Finally, we have formular:

```
MaxProfit(t) = getMax( MaxProfit(t-1), price(t) - getMax(price(t) - min, price(t) - MinC(t-1)) )
| with MinC(t-1) = getMin( MinC(t-2), price(t-1) - MaxProtfit(t-3)
```
