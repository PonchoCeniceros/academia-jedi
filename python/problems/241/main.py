import pytest
from typing import List
import re
from utils.log import Log


#
# Exercise Different Ways to Add Parentheses
#
class Solution:
    def diffWaysToCompute(self, expression: str) -> List[int]:
        # algoritmo recursivo
        def solver(E, Ops):
            if len(E) == 1:
                return E

            ans = []
            for i, op in enumerate(Ops):
                for x in solver(E[0 : i + 1], Ops[0:i]):
                    for y in solver(E[i + 1 : len(E)], Ops[i + 1 : len(Ops)]):
                        ans.append(
                            x + y if op == "+" else (x - y if op == "-" else x * y)
                        )
            return ans

        # preprocesar la entrada
        ops = ["+", "-", "*"]
        tokens = re.findall(r"(?:[1-9][0-9]|[0-9])|[+\-*]", expression)

        E = [int(t) for t in tokens if t not in ops]
        Ops = [t for t in tokens if t in ops]

        # implementacion de la funcion
        return E if len(E) == 1 else solver(E, Ops)


# Unit tests


@pytest.mark.parametrize(
    "expression, expected",
    [
        ("11", [11]),
        ("2-1-1", [0, 2]),
        ("2*3-4*5", [-34, -14, -10, -10, 10]),
    ],
)
def test_diffWaysToCompute(expression, expected):
    sol = Solution()
    result = sol.diffWaysToCompute(expression)
    assert sorted(result) == sorted(expected)
