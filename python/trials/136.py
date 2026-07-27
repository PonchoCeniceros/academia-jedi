import pytest
from typing import List, Set
from utils.log import Log


#
# Exercise Single Number
#
class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        S: Set = set()

        for n in nums:
            if n not in S:
                S.add(n)
            else:
                S.remove(n)

        return list(S)[0]


# Unit tests


@pytest.mark.parametrize(
    "nums, expected",
    [
        ([2, 2, 1], 1),
        ([4, 1, 2, 1, 2], 4),
        ([1], 1),
    ],
)
def test_singleNumber(nums, expected):
    sol = Solution()
    assert sol.singleNumber(nums) == expected
