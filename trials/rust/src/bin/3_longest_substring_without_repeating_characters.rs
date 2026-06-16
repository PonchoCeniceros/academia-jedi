use colored::*;
use katas::s;
use std::collections::HashMap;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        // @todo tal vez sea mejor usar un HashMap
        // para guardar como clave o el char o la posicion
        let mut ans = 1;
        let mut set: HashMap<char, i32> = HashMap::new();

        // @todo es posible que considere evaluar las posiciones
        // de los chars en el momento de su insercion?

        for (u, c) in s.chars().enumerate() {
            set.insert(c, u as i32);
        }

        let mut arr: Vec<i32> = set.into_values().collect();
        arr.sort();

        for i in 0..arr.len() - 1 {
            let diff = arr[i + 1] - arr[i];
            ans = if diff == 1 { ans + 1 } else { ans };
        }

        ans
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
    }
}
