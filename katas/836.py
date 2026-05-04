import pytest
from utils.log import Log
from typing import List, Dict, Optional


#
# Rectangle Overlap
#
class Solution:
    def isRectangleOverlap(self, rec1: List[int], rec2: List[int]) -> bool:
        return True


# Unit tests


@pytest.mark.parametrize(
    "rec1, rec2, expected",
    [
        ([0, 0, 2, 2], [1, 1, 3, 3], True),
        ([0, 0, 1, 1], [1, 0, 2, 1], False),
        ([0, 0, 1, 1], [2, 2, 3, 3], False),
    ],
)
def test_rectangle_overlap(rec1, rec2, expected):
    sol = Solution()
    assert sol.isRectangleOverlap(rec1, rec2) == expected
