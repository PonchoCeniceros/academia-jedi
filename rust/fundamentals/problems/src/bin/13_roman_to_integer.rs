use katas::s;
use std::collections::HashMap;

struct Solution;

/**
 * Implementa tu solución aquí
 *
 */
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let hash_map: HashMap<String, i32> = HashMap::from([
            (s!("I"), 1),
            (s!("V"), 5),
            (s!("X"), 10),
            (s!("L"), 50),
            (s!("C"), 100),
            (s!("D"), 500),
            (s!("M"), 1000),
            (s!("IX"), 9),
            (s!("XC"), 90),
            (s!("CM"), 900),
            (s!("IV"), 4),
            (s!("XL"), 40),
            (s!("CD"), 400),
        ]);

        // transformando el string en una coleccion para poder recorrerla
        let arr = s.as_bytes();
        let len = arr.len();
        let mut idx = 0;
        let mut ans = 0;

        while idx < len {
            let key = (arr[idx] as char).to_string();

            if idx + 1 < s.len() {
                match key.as_str() {
                    "I" => {
                        let next_key = (arr[idx + 1] as char).to_string();

                        match next_key.as_str() {
                            "V" => {
                                let val = hash_map.get("IV").unwrap();
                                ans += *val;
                                idx += 2;
                                continue;
                            }
                            "X" => {
                                let val = hash_map.get("IX").unwrap();
                                ans += *val;
                                idx += 2;
                                continue;
                            }
                            _ => {}
                        }
                    }

                    "X" => {
                        let next_key = (arr[idx + 1] as char).to_string();

                        match next_key.as_str() {
                            "C" => {
                                let val = hash_map.get("XC").unwrap();
                                ans += *val;
                                idx += 2;
                                continue;
                            }
                            "L" => {
                                let val = hash_map.get("XL").unwrap();
                                ans += *val;
                                idx += 2;
                                continue;
                            }
                            _ => {}
                        }
                    }

                    "C" => {
                        let next_key = (arr[idx + 1] as char).to_string();

                        match next_key.as_str() {
                            "M" => {
                                let val = hash_map.get("CM").unwrap();
                                ans += *val;
                                idx += 2;
                                continue;
                            }
                            "D" => {
                                let val = hash_map.get("CD").unwrap();
                                ans += *val;
                                idx += 2;
                                continue;
                            }
                            _ => {}
                        }
                    }

                    _ => {}
                }
            }
            //
            let val = hash_map.get(&key).unwrap();
            ans += *val;
            idx += 1;
        }

        ans
    }
}

/**
 * Pruebas unitarias
 *
 */
fn main() {
    let ans = Solution::roman_to_int(s!("IV"));
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_integer() {
        assert_eq!(Solution::roman_to_int(s!("III")), 3);
        assert_eq!(Solution::roman_to_int(s!("LVIII")), 58);
        assert_eq!(Solution::roman_to_int(s!("MCMXCIV")), 1994);
    }
}
