import pytest
from typing import Dict
from utils.log import Log


#
# Exercise Climbing Stairs
#
class Solution:
    memo: Dict = {}

    def climbStairs(self, n: int) -> int:
        # cuantas formas hay de bajar:
        #
        # 0 escalones: 1 forma, no hacer nada
        # 1 escalón:   1 forma, subir el escalon
        # 2 escalones: 2 formas,
        #             (1) subir de a 2 escalones,
        #             (2) subir de 1 en 1
        if n < 3:
            return n

        if n in self.memo:
            return self.memo[n]

        self.memo[n] = self.climbStairs(n - 1) + self.climbStairs(n - 2)
        return self.memo[n]


# Unit tests


@pytest.mark.parametrize(
    "n, expected",
    [
        (2, 2),
        (3, 3),
        (1, 1),
        (4, 5),
    ],
)
def test_climbStairs(n, expected):
    sol = Solution()
    assert sol.climbStairs(n) == expected
