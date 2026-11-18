import pytest
from typing import List, Set
from utils.log import Log


#
# Exercise Fair Candy Swap
#
class Solution:
    def fairCandySwap(self, aliceSizes: List[int], bobSizes: List[int]) -> List[int]:
        xSum = sum(a for a in aliceSizes)
        ySum = sum(b for b in bobSizes)

        xBoxes: List = aliceSizes
        yBoxes: Set = set(bobSizes)

        # al principio desarrollé el algoritmo con O(n²):
        #
        # for x in xBoxes:
        #     for y in yBoxes:
        #         if (xSum - x + y) == (ySum - y + x):
        #             ans = [x, y] if priority == "alice" else [y, x]
        #
        # ovbia/e dió LTE.
        # entonces, asumimos que para que la igualdad
        #
        #        xSum - x + y = ySum - y + x
        #
        # sea correcta, debe existir un y que satisfaga una x
        # iteramos sobre las x's y consultamos sobre las y's

        for x in xBoxes:
            y = int(x + (ySum - xSum) / 2)
            if y in yBoxes:
                return [x, y]

        return []


# Unit tests


@pytest.mark.parametrize(
    "aliceSizes, bobSizes, expected",
    [
        ([1, 1], [2, 2], [1, 2]),
        ([2], [1, 3], [2, 1]),
    ],
)
def test_fairCandySwap(aliceSizes, bobSizes, expected):
    sol = Solution()
    result = sol.fairCandySwap(aliceSizes, bobSizes)
    assert sorted(result) == sorted(expected)
