import pytest
from typing import Dict
from utils.log import Log


#
# Exercise N-th Tribonacci Number
#
class Solution:
    memo: Dict = {}

    def tribonacci(self, n: int) -> int:
        if n < 2:
            return n

        if n == 2:
            return 1

        if n in self.memo:
            return self.memo[n]

        self.memo[n] = (
            self.tribonacci(n - 3) + self.tribonacci(n - 2) + self.tribonacci(n - 1)
        )
        return self.memo[n]


# Unit tests


@pytest.mark.parametrize(
    "n, expected",
    [
        (4, 4),
        (25, 1389537),
        (0, 0),
        (1, 1),
        (2, 1),
    ],
)
def test_tribonacci(n, expected):
    sol = Solution()
    assert sol.tribonacci(n) == expected
