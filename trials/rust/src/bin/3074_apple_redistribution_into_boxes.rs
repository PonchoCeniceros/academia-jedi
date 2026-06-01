use std::cmp::Reverse;

use colored::*;
use katas::s;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
        let mut remains: i32 = apple.iter().sum();
        let mut reassigments = 0;
        let mut sorted_cap = capacity.clone();
        sorted_cap.sort();
        sorted_cap.reverse();

        let mut idx = 0;
        while remains > 0 {
            remains -= sorted_cap[idx];
            reassigments += 1;
            idx += 1;
        }

        reassigments
    }
}

fn main() {
    let ans = Solution::minimum_boxes(vec![1, 3, 2], vec![4, 3, 1, 5, 2]);
    println!("{}", format!("{}", ans).black().bold().on_bright_green());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apple_redistribution_into_boxes() {
        assert_eq!(
            Solution::minimum_boxes(vec![1, 3, 2], vec![4, 3, 1, 5, 2]),
            2
        );
        assert_eq!(Solution::minimum_boxes(vec![5, 5, 5], vec![2, 4, 2, 7]), 4);
        assert_eq!(
            Solution::minimum_boxes(vec![9, 8, 8, 2, 3, 1, 6], vec![10, 1, 4, 10, 8, 5]),
            5
        );
    }
}
