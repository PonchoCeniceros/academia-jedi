import pytest
from typing import List, Set
from utils.log import Log


#
# Exercise Remove Duplicates from Sorted Array
#
class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        # implementamos un set auxiliar para alamcenar los elementos unicos
        # y una lista auxiliar para procesar el algoritmo (muy programación funcional)
        aux: Set = set()
        arr: List = []

        for n in nums:
            if n not in aux:
                aux.add(n)
                arr.append(n)

        # este truco me gustó, cambiar el contenido
        # completo de la lista con un slice
        nums[:] = arr
        return len(nums)


# Unit tests


@pytest.mark.parametrize(
    "nums, expectedNums, expectedK",
    [
        ([1, 1, 2], [1, 2], 2),
        ([0, 0, 1, 1, 1, 2, 2, 3, 3, 4], [0, 1, 2, 3, 4], 5),
    ],
)
def test_removeDuplicates(nums, expectedNums, expectedK):
    sol = Solution()
    k = sol.removeDuplicates(nums)
    assert k == expectedK
    for i in range(k):
        assert nums[i] == expectedNums[i]
