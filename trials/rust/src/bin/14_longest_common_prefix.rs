use colored::*;
use katas::s;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut len = 0_usize;
        for word in strs {
            if len < word.len() {
                len = word.len();
            }
        }

        "".to_string()
    }
}

fn main() {
    // let ans = Solution::longest_common_prefix();
    println!(
        "{}",
        format!("{}", s!("May the Force be with you!"))
            .black()
            .bold()
            .on_bright_green()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            Solution::longest_common_prefix(vec![s!("flower"), s!("flow"), s!("flight")]),
            s!("fl")
        );
    }
}
