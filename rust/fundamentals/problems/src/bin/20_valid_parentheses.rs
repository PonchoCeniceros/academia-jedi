use colored::*;
use katas::s;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    // | Carácter | ASCII |
    // | -------- | ----- |
    // | `(`      | 40    |
    // | `)`      | 41    |
    // | `[`      | 91    |
    // | `]`      | 93    |
    // | `{`      | 123   |
    // | `}`      | 125   |

    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<u8> = Vec::new();
        let s_arr = s.as_bytes();

        if s_arr[0] == 41 || s_arr[0] == 93 || s_arr[0] == 125 {
            return false;
        }

        for i in s_arr.iter() {
            if *i == 40 || *i == 91 || *i == 123 {
                stack.push(*i);
            } else {
                if !stack.is_empty() {
                    let top = stack.last();
                    if (top == Some(&40) && *i == 41)
                        || (top == Some(&91) && *i == 93)
                        || (top == Some(&123) && *i == 125)
                    {
                        stack.pop();
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}

fn main() {
    let ans = Solution::is_valid(s!("()"));
    println!("{}", format!("{}", ans).black().bold().on_bright_green());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_parentheses() {
        assert!(Solution::is_valid(s!("()")));
        assert!(Solution::is_valid(s!("()[]{}")));
        assert!(!Solution::is_valid(s!("(]")));
        assert!(Solution::is_valid(s!("([])")));
        assert!(!Solution::is_valid(s!("([)]")));
        assert!(!Solution::is_valid(s!("(){}}{")));
        assert!(!Solution::is_valid(s!("(])")));
    }
}
