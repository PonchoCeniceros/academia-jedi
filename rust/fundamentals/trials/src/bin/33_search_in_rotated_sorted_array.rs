use colored::*;

struct Solution;

/**
 * Implement your solution here
 */
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        0
    }
}

fn main() {
    let ans = Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0);
    println!("{}", format!("{}", ans).green().italic().underline());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_in_rotated_sorted_array() {
        let cases = [
            ((vec![4, 5, 6, 7, 0, 1, 2], 0), 4),
            ((vec![4, 5, 6, 7, 0, 1, 2], 3), -1),
            ((vec![1], 0), -1),
        ];

        for (input, expected) in cases {
            assert_eq!(
                Solution::search(input.0.clone(), input.1),
                expected,
                "{}",
                format!("{:?}", input).red().italic().underline()
            );
        }
    }
}
