use colored::*;

struct Solution;

/**
 * Implement your solution here
 */
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            let idx = mid as usize;

            if target == nums[idx] {
                return mid;
            }

            if target < nums[idx] {
                right = mid - 1;
            }

            if target > nums[idx] {
                left = mid + 1;
            }
        }

        -1
    }
}

fn main() {
    let ans = Solution::search(vec![-1, 0, 3, 5, 9, 12], 9);
    println!("{}", format!("{}", ans).green().italic().underline());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let cases = [
            ((vec![-1, 0, 3, 5, 9, 12], 9), 4),
            ((vec![-1, 0, 3, 5, 9, 12], 2), -1),
        ];

        for (input, expected) in cases {
            assert_eq!(
                Solution::search(input.0.clone(), input.1),
                expected,
                "{}",
                format!("{:?}", input.0).red().italic().underline()
            );
        }
    }
}
