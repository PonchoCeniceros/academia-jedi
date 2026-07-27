import pytest
from typing import List
from utils.log import Log


#
# Exercise Maximum Product of Two Elements in an Array
#
class Solution:
    def maxProduct(self, nums: List[int]) -> int:
        u, iu, v = -1, -1, -1

        for i, n in enumerate(nums):
            if n > u:
                u, iu = n, i

        for i, n in enumerate(nums):
            if iu != i:
                if n > v:
                    v = n

        return (u - 1) * (v - 1)


# Unit tests


@pytest.mark.parametrize(
    "nums, expected",
    [
        ([3, 4, 5, 2], 12),
        ([1, 5, 4, 5], 16),
        ([3, 7], 12),
    ],
)
def test_maxProduct(nums, expected):
    sol = Solution()
    assert sol.maxProduct(nums) == expected
