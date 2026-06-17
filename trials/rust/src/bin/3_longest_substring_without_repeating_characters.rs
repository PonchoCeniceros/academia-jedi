use colored::*;
use katas::s;
use std::{cmp::max, collections::HashSet};

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // arreglo de chars en lugar de String
        let mut i = 0;
        let arr: Vec<char> = s.chars().collect();
        // calcular la longitud de los substrs y el maximo de ellos
        let (mut l, mut max_l) = (0, 0);
        // set para ir guardando los chars de la substr consultada
        let mut set: HashSet<char> = HashSet::new();

        loop {
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
            }
        }

        max_l
    }
}

fn main() {
    let s = s!(" ");
    let ans = Solution::length_of_longest_substring(s);
    println!("{}", format!("{}", ans).black().bold().on_bright_green());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(Solution::length_of_longest_substring(s!("abcabcbb")), 3);
        assert_eq!(Solution::length_of_longest_substring(s!("bbbbb")), 1);
        assert_eq!(Solution::length_of_longest_substring(s!("pwwkew")), 3);
        assert_eq!(Solution::length_of_longest_substring(s!("aab")), 2);
        assert_eq!(Solution::length_of_longest_substring(s!("dvdf")), 3);
        assert_eq!(Solution::length_of_longest_substring(s!("asjrgapa")), 6);
    }
}
