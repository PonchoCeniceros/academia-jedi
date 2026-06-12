use colored::*;
use katas::s;
use std::cmp::min;
use std::collections::HashMap;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn c(s: i32, cost: &[i32]) -> bool {
        s >= cost.len() as i32
    }

    pub fn compute(s: i32, c: i32, cost: &[i32], memo: &mut HashMap<i32, i32>) -> i32 {
        if Solution::c(s, cost) {
            return 0;
        }

        if let Some(&ans) = memo.get(&s) {
            return ans;
        }

        let x = if Solution::c(s + 1, cost) {
            0
        } else {
            cost[(s + 1) as usize]
        };

        let y = if Solution::c(s + 2, cost) {
            0
        } else {
            cost[(s + 2) as usize]
        };

        let ans = c + min(
            Solution::compute(s + 1, x, cost, memo),
            Solution::compute(s + 2, y, cost, memo),
        );

        memo.insert(s, ans);
        ans
    }

    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut memo: HashMap<i32, i32> = HashMap::new();
        Solution::compute(-1, 0, &cost, &mut memo)
    }
}

fn main() {
    let ans = Solution::min_cost_climbing_stairs(vec![10, 15, 20]);
    println!("{}", format!("{}", ans).black().bold().on_bright_green());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost_climbing_stairs() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
