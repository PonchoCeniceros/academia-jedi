import pytest
from typing import List
from utils.log import Log


#
# Exercise Counting Bits
#
class Solution:
    def count(self, n: int) -> int:
        x, s = n, 0
        while x > 0:
            s += x % 2
            x = x // 2
        return s

    def countBits(self, n: int) -> List[int]:
        ans: List = []
        for i in range(n + 1):
            s = self.count(i)
            ans.append(s)
        return ans


# Unit tests


@pytest.mark.parametrize(
    "n, expected",
    [
        (2, [0, 1, 1]),
        (5, [0, 1, 1, 2, 1, 2]),
    ],
)
def test_countBits(n, expected):
    sol = Solution()
    assert sol.countBits(n) == expected
