import pytest
from utils.log import Log
from typing import List, Dict, Optional


#
# Find the Index of the First Occurrence in a String
#
class Solution:
    def strStr(self, haystack: str, needle: str) -> int:
        return haystack.find(needle)


# Unit tests


@pytest.mark.parametrize(
    "haystack, needle, expected",
    [
        # Add your test cases here
        ("sadbutsad", "sad", 0),
        ("leetcode", "leeto", -1),
    ],
)
def test_Find_the_Index_of_the_First_Occurrence_in_a_String(haystack, needle, expected):
    sol = Solution()
    assert sol.strStr(haystack, needle) == expected
