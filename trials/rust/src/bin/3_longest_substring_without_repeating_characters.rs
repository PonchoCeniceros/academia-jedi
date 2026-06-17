use colored::*;
use katas::s;
use std::collections::{HashMap, HashSet};

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    //
    // calcular los saltos que hay en el arreglo de
    // "posiciones" de cada letra del substring
    //
    fn compute_len(arr: &[i32]) -> i32 {
        let mut ans = 1;
        for i in 0..arr.len() - 1 {
            let diff = arr[i + 1] - arr[i];
            ans = if diff == 1 { ans + 1 } else { ans };
        }

        ans
    }

    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let mut set: HashSet<char> = HashSet::new();
        let mut map: HashMap<char, i32> = HashMap::new();

        for (u, c) in s.chars().enumerate() {
            if !set.contains(&c) {
                set.insert(c);
                map.insert(c, u as i32);
            } else {
                map.insert(c, u as i32);
            }
        }

        let mut idx: Vec<i32> = map.into_values().collect();
        idx.sort();
        Solution::compute_len(&idx)
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
    }
}
