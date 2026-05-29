## Exercise 746

In this problem I had a couple of opportunities for optimization:
1. the self.memo
2. the use of -1 as index

There is the code with corrections:

```python
def minCostClimbingStairsClimbingStairs(self, cost: List[int]) -> int:
  memo = {}

  def solve(i):
    if i >= len(cost):
      return 0

    if i in memo:
      return memo[i]

    memo[i] = cost[i] + min(solve(i + 1), solve(i + 2))
    return memo[i]

  return min(solve(0), solve(1))
```

And we could use the `@cache` decorator for the memoization:

```python
from functools import cache

def minCostClimbingStairs(self, cost: List[int]) -> int:

  @cache
  def solve(i):
      if i >= len(cost):
        return 0

    return cost[i] + min(solve(i + 1), solve(i + 2))

  return min(solve(0), solve(1))
```
