use colored::*;
use katas::s;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut ans = "".to_string();
        let mut len = 2001_usize;
        for word in strs.iter() {
            if len > word.len() {
                len = word.len();
            }
        }
        for i in 0..len {
            let mut tmp = 0_u8;
            for (j, word) in strs.iter().enumerate() {
                if j == 0 {
                    tmp = word.as_bytes()[i];
                } else {
                    tmp = if tmp != word.as_bytes()[i] { 0 } else { tmp }
                }
            }

            if tmp == 0 {
                return ans;
            }
            ans.push(tmp as char);
        }
        ans
    }
}

fn main() {
    let ans = Solution::longest_common_prefix(vec![s!("flower"), s!("flow"), s!("flight")]);
    println!("{}", ans.black().bold().on_bright_green());
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
