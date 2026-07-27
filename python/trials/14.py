import pytest
from utils.log import Log
from typing import List


#
# Longest Common Prefix
#
class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        if len(strs) == 0:
            return ""

        lens = [len(arr) for arr in strs]
        minLen = min(lens)

        ans = ""
        bfr = ""

        for i in range(minLen):
            for j, s in enumerate(strs):
                # Log.teal(f"{j}, {i}, {s[i]}")
                if j == 0:
                    bfr = s[i]
                else:
                    bfr = "" if bfr != s[i] else s[i]
                # Log.cyan(bfr)
            if bfr == "":
                return ans
            ans = ans + bfr
        # Log.blue(f"{ans}")
        return ans


# Unit tests


@pytest.mark.parametrize(
    "strs, expected",
    [
        (["flower", "flow", "flight"], "fl"),
        (["dog", "racecar", "car"], ""),
        ([], ""),
        (["cir", "car"], "c"),
    ],
)
def test_Longest_Common_Prefix(strs, expected):
    sol = Solution()
    assert sol.longestCommonPrefix(strs) == expected
