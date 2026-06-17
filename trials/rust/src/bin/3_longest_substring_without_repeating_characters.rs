use colored::*;
use katas::s;
use std::{cmp::max, collections::HashSet};

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    fn get_max_len(arr: &[char]) -> i32 {
        // set para ir guardando los chars de la substr consultada
        let mut set: HashSet<char> = HashSet::new();

        // indices para recorrer el arreglo de chars
        let (mut r, mut i) = (0_usize, 0_usize);
        // calcular la longitud de los substrs y el maximo de ellos
        let (mut l, mut max_l) = (0, 0);

        loop {
            // mi substr empieza aqui
            if i >= arr.len() {
                max_l = max(max_l, l);
                break;
            }

            if !set.contains(&arr[i]) {
                set.insert(arr[i]);
                l += 1;
                i += 1;
            } else {
                max_l = max(max_l, l);
                set.clear();
                l = 0;
                // y mi substr termina aqui
                if arr[i] == arr[r] && (i as i32 - r as i32) > 1 {
                    i -= 1;
                }
                // reacomodo r al nuevo substr
                r = i;
            }
        }

        max_l
    }

    pub fn length_of_longest_substring(s: String) -> i32 {
        // arreglo de chars en lugar de String
        let mut arr: Vec<char> = s.chars().collect();

        if arr.len() <= 1 {
            return arr.len() as i32;
        }

        let a = Solution::get_max_len(&arr);
        arr.reverse();
        let b = Solution::get_max_len(&arr);

        max(a, b)
    }
}

fn main() {
    let s = s!("aab");
    let ans = Solution::length_of_longest_substring(s);
    println!("{}", format!("{}", ans).black().bold().on_bright_green());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        let cases = [
            ("abcabcbb", 3),
            ("bbbbb", 1),
            ("pwwkew", 3),
            ("aab", 2),
            ("dvdf", 3),
            ("asjrgapa", 6),
            (" ", 1),
            ("busvutpwmu", 7),
        ];

        for (input, expected) in cases {
            assert_eq!(
                Solution::length_of_longest_substring(s!(input)),
                expected,
                "{}",
                format!("{:?}", input).red().italic().underline()
            );
        }
    }
}
