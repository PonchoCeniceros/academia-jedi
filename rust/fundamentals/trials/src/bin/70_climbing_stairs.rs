use std::collections::HashMap;

use colored::*;
use katas::s;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn compute(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if n < 3 {
            return n;
        }

        if let Some(&ans) = memo.get(&n) {
            return ans;
        }

        let n1 = Solution::compute(n - 1, memo);
        let n2 = Solution::compute(n - 2, memo);
        memo.insert(n, n1 + n2);
        n1 + n2
    }

    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo: HashMap<i32, i32> = HashMap::new();
        Solution::compute(n, &mut memo)
    }
}

fn main() {
    let ans = Solution::climb_stairs(2);
    println!("{}", format!("{}", ans).black().bold().on_bright_green());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climbing_stairs() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(1), 1);
        assert_eq!(Solution::climb_stairs(4), 5);
    }
}
