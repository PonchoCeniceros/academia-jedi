use colored::*;
use katas::s;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<u8> = vec![];

        if s.len() < 3 {
            return false;
        }

        for c in s.as_bytes().iter() {
            stack.push(*c);
            if stack.len() >= 3 {
                let last = stack.len() - 1;
                let chain = format!(
                    "{}{}{}",
                    stack[last - 2] as char,
                    stack[last - 1] as char,
                    stack[last] as char
                );
                if chain == "abc" {
                    stack.pop();
                    stack.pop();
                    stack.pop();
                }
            }
        }
        stack.is_empty()
    }
}

fn main() {
    let ans = Solution::is_valid(s!("abccba"));
    println!("{}", format!("{}", ans).black().bold().on_bright_green());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_if_word_valid_after_substitutions() {
        assert!(Solution::is_valid(s!("aabcbc")));
        assert!(Solution::is_valid(s!("abcabcababcc")));
        assert!(!Solution::is_valid(s!("abccba")));
    }
}
