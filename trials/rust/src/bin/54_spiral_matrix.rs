use colored::*;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        vec![0]
    }
}

fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let ans = Solution::spiral_order(matrix);
    println!("{:?}", format!("{:?}", ans).green().italic().underline());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_matrix() {
        let cases = [
            (
                vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
                vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
            ),
            (
                vec![
                    vec![1, 2, 3, 4],
                    vec![5, 6, 7, 8],
                    vec![9, 10, 11, 12],
                    vec![13, 14, 15, 16],
                ],
                vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
            ),
        ];

        for (input, expected) in cases {
            assert_eq!(
                Solution::spiral_order(input.clone()),
                expected,
                "{}",
                format!("{:?}", input).red().italic().underline()
            );
        }
    }
}
