use colored::*;
use katas::s;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        0
    }
}

fn main() {
    let ans = Solution::coin_change(vec![1, 2, 5], 11);
    println!("{}", format!("{}", ans).green().italic().underline());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_change() {
        let cases = [
            ((vec![1, 2, 5], 11), 3),
            ((vec![2], 3), -1),
            ((vec![1], 0), 0),
        ];

        for (input, expected) in cases {
            assert_eq!(
                Solution::coin_change(input.0.clone(), input.1),
                expected,
                "{}",
                format!("{:?}", input.0).red().italic().underline()
            );
        }
    }
}
