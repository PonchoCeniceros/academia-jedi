struct Solution;
use std::collections::HashMap;

/**
 * Implementa tu solución aquí
 *
 */
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let hash_map: HashMap<String, i32> = HashMap::from([
            (String::from("I"), 1),
            (String::from("V"), 5),
            (String::from("X"), 10),
            (String::from("L"), 50),
            (String::from("C"), 100),
            (String::from("D"), 500),
            (String::from("M"), 1000),
            (String::from("IX"), 9),
            (String::from("XC"), 90),
            (String::from("CM"), 900),
            (String::from("IV"), 4),
            (String::from("XL"), 40),
            (String::from("CD"), 400),
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
    let ans = Solution::roman_to_int(String::from("IV"));
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_integer() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }
}
