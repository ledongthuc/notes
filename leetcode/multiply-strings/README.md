# Add Binary

https://leetcode.com/problems/add-binary/

## Summary

Use same solution we use in basic multiplication method:

![enter image description here](https://www.trimleystmartinprimaryschool.com/uploads/2/7/3/5/27353585/img-7510_orig.jpeg)

Split the problem to two smaller problems:

 - Problem 1: Mutiply each digits column of first number with second number.
	 - Sequencely, each digits of first number multiply each second digits number.
	 - If result of digits multiplication is greater than 10, carried tens digit to next level.

![MultiplyStrings-Step1](https://user-images.githubusercontent.com/1828895/120063342-01d78c80-c067-11eb-8226-cbf5996a48ba.png)

 - Problem 2: Sum all result from problem 1.

![MultiplyStrings-Step2](https://user-images.githubusercontent.com/1828895/120063353-0dc34e80-c067-11eb-8da0-921e1a985001.png)

```go
func multiply(num1 string, num2 string) string {
    if num1 == "0" || num2 == "0" {
        return "0"
    }
    resultsToSum := multiplyUnits(num1, num2)
    return sum(resultsToSum)
}

func multiplyUnits(num1 string, num2 string) []string {
    resultsToSum := make([]string, 0, len(num1) * len(num2))
    
    for i1 := len(num1) - 1; i1 >= 0; i1-- {
        n1 := charToInt(num1[i1])
        
        resultToSum := ""
        var remaining, unitResult int
        for i2 := len(num2) - 1; i2 >= 0; i2-- {
            n2 := charToInt(num2[i2])
            
            remaining, unitResult = getParts((n1 * n2 + remaining))
            resultToSum = intToChar(unitResult) + resultToSum
        }
        
        if remaining > 0 {
            resultToSum = intToChar(remaining) + resultToSum
        }
        
        column := len(num1) - i1 - 1
        for cIndex := 0; cIndex < column; cIndex++{
           resultToSum += "0" 
        }
        
        resultsToSum = append(resultsToSum, resultToSum)
        column++
    }
    
    return resultsToSum
}

func sum(nums []string) string {
    maxL := getMaxLength(nums)
    var result string
    var remaining, unit int
    for i := 0; i < maxL; i++ {
        for _, num := range nums {
            unitIndex := len(num) - 1 - i
            if unitIndex >= 0 {
                remaining += charToInt(num[unitIndex])
            }
        }
        
        remaining, unit = getParts(remaining)
        
        result = intToChar(unit) + result
    }
    
    if remaining > 0 {
        result = intToChar(remaining) + result
    }
    return result
}

func charToInt(c byte) int {
    return int(c - 48)
}

func intToChar(i int) string {
    return string(rune(i + 48))
}

func getParts(n int) (tens, unit int) {
    return n/10, n%10
}

func getMaxLength(nums []string) int {
    l := 0
    for _, num := range nums {
        if l < len(num) {
            l = len(num)
        }
    }
    return l
}
```
