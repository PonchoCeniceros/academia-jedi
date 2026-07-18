use colored::*;
use std::cmp::max;

struct Solution;

/**
 * Implement your solution here
 */
impl Solution {
    fn solve(n: usize, nums: &[i32]) -> i32 {
        if n >= nums.len() {
            return 0;
        }

        nums[n] + Solution::solve(n + 2, nums)
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        max(Solution::solve(0, &nums), Solution::solve(1, &nums))
    }
}

fn main() {
    let ans = Solution::rob(vec![1, 2, 3, 1]);
    println!("{}", format!("{}", ans).green().italic().underline());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_house_robber() {
        let cases = [(vec![1, 2, 3, 1], 4), (vec![2, 7, 9, 3, 1], 12)];

        for (input, expected) in cases {
            assert_eq!(
                Solution::rob(input.clone()),
                expected,
                "{}",
                format!("{:?}", input).red().italic().underline()
            );
        }
    }
}
