import pytest
from utils.log import Log
from typing import List, Dict, Optional


#
# Palindrome Number
#
class Solution:
    def isPalindrome(self, x: int) -> bool:
        if x < 0:
            return False

        n = [int(i) for i in str(x)]
        r = n[::-1]

        for i in range(len(n)):
            # Log.magenta(f"{n[i]} {r[i]}")
            if n[i] != r[i]:
                return False

        return True


# Unit tests


@pytest.mark.parametrize(
    "x, expected",
    [(121, True), (-121, False), (10, False)],
)
def test_Palindrome_Number(x, expected):
    sol = Solution()
    assert sol.isPalindrome(x) == expected
