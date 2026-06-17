use colored::*;
use katas::s;
use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    fn get_len(arr: &[char], idx: &mut usize) -> i32 {
        let mut l = 0;
        let mut set: HashSet<char> = HashSet::new();

        loop {
            if *idx >= arr.len() {
                break;
            }

            let c = arr[*idx];
            if !set.contains(&c) {
                set.insert(c);
                l += 1;
                *idx += 1;
            } else {
                return l;
            }
        }

        l
    }

    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut l = 0;
        let mut idx = 0;
        let mut arr: Vec<char> = s.chars().collect();

        loop {
            if idx >= arr.len() {
                break;
            }
            l = max(Solution::get_len(&arr, &mut idx), l);
        }

        idx = 0;
        arr.reverse();

        loop {
            if idx >= arr.len() {
                break;
            }
            l = max(Solution::get_len(&arr, &mut idx), l);
        }

        l
    }
}

fn main() {
    let s = s!("abcabcbb");
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
    }
}
