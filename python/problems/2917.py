import pytest
from typing import List
from utils.log import Log


#
# Exercise Find the K-or of an Array
#
class Solution:
    def findKOr(self, nums: List[int], k: int) -> int:
        answ = 0
        mask = 1

        for _ in range(32):
            cnt = 0
            for n in nums:
                bit = n & mask
                cnt = cnt + 1 if bit > 0 else cnt
            if cnt >= k:
                answ += mask

            mask <<= 1
        return answ


# Unit tests


@pytest.mark.parametrize(
    "nums, k, expected",
    [
        ([7, 12, 9, 8, 9, 15], 4, 9),
        ([2, 12, 1, 11, 4, 5], 6, 0),
        ([10, 8, 5, 9, 11, 6, 8], 1, 15),
    ],
)
def test_findKOr(nums, k, expected):
    sol = Solution()
    assert sol.findKOr(nums, k) == expected
