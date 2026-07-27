import pytest
from utils.log import Log


#
# Exercise Number of 1 Bits
#
class Solution:
    def hammingWeight(self, n: int) -> int:
        x, s = n, 0
        while x > 0:
            s += x % 2
            x = x // 2
        return s


# Unit tests


@pytest.mark.parametrize(
    "n, expected",
    [
        (11, 3),
        (128, 1),
        (2147483645, 30),
    ],
)
def test_hammingWeight(n, expected):
    sol = Solution()
    assert sol.hammingWeight(n) == expected
