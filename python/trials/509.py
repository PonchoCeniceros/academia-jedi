import pytest
from typing import Dict
from utils.log import Log


#
# Exercise Fibonacci Number
#
class Solution:
    memo: Dict = {}

    def fib(self, n: int) -> int:
        #
        # caso(s) base
        #
        if n < 2:
            return n

        # veo si el resultado de este caso ya fué
        # calculado por otra llamada recusiva previa
        #
        if n in self.memo:
            return self.memo[n]

        # calculo el resultado y lo almaceno en memoria
        #
        self.memo[n] = self.fib(n - 1) + self.fib(n - 2)
        return self.memo[n]


# Unit tests


@pytest.mark.parametrize(
    "n, expected",
    [
        (2, 1),
        (3, 2),
        (4, 3),
        (0, 0),
        (1, 1),
    ],
)
def test_fib(n, expected):
    sol = Solution()
    assert sol.fib(n) == expected
