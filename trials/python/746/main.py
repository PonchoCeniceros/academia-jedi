import pytest
from typing import List, Dict
from utils.log import Log


#
# Exercise Min Cost Climbing Stairs
#
class Solution:
    memo: Dict = {}

    def C(self, s, cost):
        return s >= len(cost)

    def foo(self, s: int, c: int, cost: List[int]) -> int:
        if self.C(s, cost):
            return 0

        if s in self.memo:
            return self.memo[s]

        x = 0 if self.C(s + 1, cost) else cost[s + 1]
        y = 0 if self.C(s + 2, cost) else cost[s + 2]

        self.memo[s] = c + min(
            self.foo(s + 1, x, cost),
            self.foo(s + 2, y, cost),
        )
        return self.memo[s]

    def minCostClimbingStairs(self, cost: List[int]) -> int:
        ans = self.foo(-1, 0, cost)
        self.memo.clear()
        return ans


# Unit tests


@pytest.mark.parametrize(
    "cost, expected",
    [
        ([10, 15, 20], 15),
        ([1, 100, 1, 1, 1, 100, 1, 1, 100, 1], 6),
    ],
)
def test_minCostClimbingStairs(cost, expected):
    sol = Solution()
    assert sol.minCostClimbingStairs(cost) == expected
