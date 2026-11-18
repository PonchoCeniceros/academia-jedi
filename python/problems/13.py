import pytest
from utils.log import Log
from typing import List, Dict, Optional


#
# Roman to Integer
#
# Symbol       Value
# I             1
# V             5
# X             10
# L             50
# C             100
# D             500
# M             1000
#
# * I can be placed before V (5) and X (10) to make 4 and 9.
# * X can be placed before L (50) and C (100) to make 40 and 90.
# * C can be placed before D (500) and M (1000) to make 400 and 900.
#


class Solution:
    table = {
        "I": 1,
        "V": 5,
        "X": 10,
        "L": 50,
        "C": 100,
        "D": 500,
        "M": 1000,
        "IX": 9,
        "XC": 90,
        "CM": 900,
        "IV": 4,
        "XL": 40,
        "CD": 400,
    }

    def romanToInt(self, s: str) -> int:
        ans = 0
        i = 0

        while i < len(s):
            if s[i] == "I":
                if i + 1 < len(s) and s[i + 1] == "V":
                    # Log.blue(f"{self.table['IV']}")
                    ans = ans + self.table["IV"]
                    i = i + 2
                elif i + 1 < len(s) and s[i + 1] == "X":
                    # Log.blue(f"{self.table['IX']}")
                    ans = ans + self.table["IX"]
                    i = i + 2
                else:
                    # Log.blue(f"{self.table[s[i]]}")
                    ans = ans + self.table[s[i]]
                    i = i + 1

            elif s[i] == "X":
                if i + 1 < len(s) and s[i + 1] == "L":
                    # Log.blue(f"{self.table['XL']}")
                    ans = ans + self.table["XL"]
                    i = i + 2
                elif i + 1 < len(s) and s[i + 1] == "C":
                    # Log.blue(f"{self.table['XC']}")
                    ans = ans + self.table["XC"]
                    i = i + 2
                else:
                    # Log.blue(f"{self.table[s[i]]}")
                    ans = ans + self.table[s[i]]
                    i = i + 1

            elif s[i] == "C":
                if i + 1 < len(s) and s[i + 1] == "D":
                    # Log.blue(f"{self.table['CD']}")
                    ans = ans + self.table["CD"]
                    i = i + 2
                elif i + 1 < len(s) and s[i + 1] == "M":
                    # Log.blue(f"{self.table['CM']}")
                    ans = ans + self.table["CM"]
                    i = i + 2
                else:
                    # Log.blue(f"{self.table[s[i]]}")
                    ans = ans + self.table[s[i]]
                    i = i + 1

            else:
                # Log.blue(f"{self.table[s[i]]}")
                ans = ans + self.table[s[i]]
                i = i + 1
        return ans


# Unit tests


@pytest.mark.parametrize(
    "s, expected",
    [
        ("III", 3),
        ("LVIII", 58),
        ("MCMXCIV", 1994),
    ],
)
def test_roman_to_integer(s, expected):
    sol = Solution()
    assert sol.romanToInt(s) == expected
