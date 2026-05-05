struct Solution;
use std::collections::HashMap;

/**
 * Implementa tu solución aquí
 *
 */
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let hash_map: HashMap<&str, i32> = HashMap::from([
            ("I", 1),
            ("V", 5),
            ("X", 10),
            ("L", 50),
            ("C", 100),
            ("D", 500),
            ("M", 1000),
            ("IX", 9),
            ("XC", 90),
            ("CM", 900),
            ("IV", 4),
            ("XL", 40),
            ("CD", 400),
        ]);

        let arr = s.as_bytes();
        let len = arr.len();
        let mut idx = 0;

        while idx < len {
            let val = (arr[idx] as char).to_string();
            let ans = hash_map.get(val.as_str()).unwrap();
            println!("{}", ans);
            idx += 1;
        }

        0
    }
}

/**
 * Pruebas unitarias
 *
 */
fn main() {
    let _ans = Solution::roman_to_int("MCMXCIV".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_integer() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 0);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 0);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 0);
    }
}

/*
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
*/
