import pytest
from typing import List
from utils.log import Log


#
# Exercise Apple Redistribution into Boxes
#
class Solution:
    def minimumBoxes(self, apple: List[int], capacity: List[int]) -> int:
        remains = 0
        reassigments = 0
        capacity.sort(reverse=True)

        for a in apple:
            remains += a

        i = 0
        while remains > 0:
            remains -= capacity[i]
            reassigments += 1
            i += 1

        return reassigments


# Unit tests


@pytest.mark.parametrize(
    "apple, capacity, expected",
    [
        ([1, 3, 2], [4, 3, 1, 5, 2], 2),
        ([5, 5, 5], [2, 4, 2, 7], 4),
        ([9, 8, 8, 2, 3, 1, 6], [10, 1, 4, 10, 8, 5], 5),
    ],
)
def test_minimumBoxes(apple, capacity, expected):
    sol = Solution()
    assert sol.minimumBoxes(apple, capacity) == expected
