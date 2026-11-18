import pytest
from typing import List, Dict
from utils.log import Log


#
# Exercise Two Sum
#
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        x, y = 0, 0
        # con los hashTable podemos hacer consultas en O(1),
        # si la respuesta que buscamoas esta contenida en el
        # mismo conjunto de información, podemos usar un hash
        # map para auxiliarnos en consultar si dicha respuesta
        # ya está en el conjunto previo que hemos revisado
        hashTable: Dict = {}

        for i, x in enumerate(nums):
            y = target - x

            # necesito encontrar una y en mi hash table de x acumuladas,
            # ya que alguna de esas x satisfarán x + y = target
            if hashTable.get(y, None) is not None:
                x, y = i, hashTable[y]
                break

            # guardo la x para la siguiente y propuesta
            hashTable[x] = i

        return [x, y]


# Unit tests


@pytest.mark.parametrize(
    "nums, target, expected",
    [
        ([2, 7, 11, 15], 9, [1, 0]),
        ([3, 2, 4], 6, [2, 1]),
        ([3, 3], 6, [1, 0]),
        ([3, 2, 3], 6, [2, 0]),
    ],
)
def test_twoSum(nums, target, expected):
    sol = Solution()
    result = sol.twoSum(nums, target)
    assert sorted(result) == sorted(expected)
