import pytest
from typing import List
from utils.log import Log


#
# Exercise Valid Parentheses
#
class Solution:
    def isValid(self, s: str) -> bool:
        stack: List = []

        # if we start the string with an close tag, fail
        if s and (s[0] == ")" or s[0] == "]" or s[0] == "}"):
            return False

        for i in s:
            if i == "(" or i == "[" or i == "{":
                stack.append(i)
            else:
                if len(stack) != 0:
                    top = stack[-1]
                    if (
                        (top == "(" and i == ")")
                        or (top == "[" and i == "]")
                        or (top == "{" and i == "}")
                    ):
                        stack.pop()
                    else:
                        # if we cant close the current tag, the while chain is fail
                        return False
                else:
                    # if any char remains in the string without any open tag to close, fail
                    return False

        # if we have only open tags in the chain, fail
        return len(stack) == 0


# Unit tests


@pytest.mark.parametrize(
    "s, expected",
    [
        ("()", True),
        ("()[]{}", True),
        ("(]", False),
        ("([])", True),
        ("([)]", False),
        ("(){}}{", False),
        ("(])", False),
    ],
)
def test_isValid(s, expected):
    sol = Solution()
    assert sol.isValid(s) == expected
