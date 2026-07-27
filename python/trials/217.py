import pytest
from typing import List, Dict
from utils.log import Log


#
# Exercise Contains Duplicate
#
class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        # la verdad aqui pude haber implementado me jor un set,
        # ya que no requerimos un valor dummy para el set,
        # pero para el caso funciona
        hashTable: Dict = {}

        for n in nums:
            if n in hashTable:
                return True
            hashTable[n] = "x"  # la x es dummy
        return False


# Unit tests


@pytest.mark.parametrize(
    "nums, expected",
    [
        ([1, 2, 3, 1], True),
        ([1, 2, 3, 4], False),
        ([1, 1, 1, 3, 3, 4, 3, 2, 4, 2], True),
    ],
)
def test_containsDuplicate(nums, expected):
    sol = Solution()
    assert sol.containsDuplicate(nums) == expected
