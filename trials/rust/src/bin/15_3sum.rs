use colored::*;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        vec![vec![0]]
    }
}

fn main() {
    let nums = vec![0, 1, 1];
    let ans = Solution::three_sum(nums);
    println!("{}", format!("{:?}", ans).green().italic().underline());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        let cases: [(Vec<i32>, Vec<Vec<i32>>); 3] = [
            (vec![0, 1, 1], vec![]),
            (vec![0, 0, 0], vec![vec![0, 0, 0]]),
            (
                vec![-1, 0, 1, 2, -1, -4],
                vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            ),
        ];

        for (input, expected) in cases {
            assert_eq!(
                Solution::three_sum(input.clone()),
                expected,
                "{}",
                format!("{:?}", input).red().italic().underline()
            );
        }
    }
}
