use std::collections::HashSet;

use colored::*;
use katas::s;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let x_sum: i32 = alice_sizes.iter().sum();
        let y_sum: i32 = bob_sizes.iter().sum();

        let x_boxes = alice_sizes;
        let y_boxes: HashSet<i32> = bob_sizes.into_iter().collect();

        for x in x_boxes.iter() {
            let y = *x + (y_sum - x_sum) / 2;
            if y_boxes.contains(&y) {
                return vec![*x, y];
            }
        }

        vec![]
    }
}

fn main() {
    let ans = Solution::fair_candy_swap(vec![1, 1], vec![2, 2]);
    let ans = Solution::fair_candy_swap(vec![2], vec![1, 3]);
    println!("{:?}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fair_candy_swap() {
        assert_eq!(
            Solution::fair_candy_swap(vec![1, 1], vec![2, 2]),
            vec![1, 2]
        );
        assert_eq!(Solution::fair_candy_swap(vec![2], vec![1, 3]), vec![2, 3]);
    }
}
