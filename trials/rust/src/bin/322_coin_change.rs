use colored::*;
use std::{char::MAX, cmp::min};

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    fn solve(remainder: i32, coins: &[i32]) -> i32 {
        if remainder <= 0 {
            return 0;
        }

        let mut ans = i32::MAX;

        for &coin_val in coins.iter() {
            let coin_qty = 1 + Solution::solve(remainder - coin_val, coins);
            ans = min(ans, coin_qty);
        }
        ans
    }

    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // PA: ¿cual es mi estado actual?
        // PB: ¿que decisiones puedo tomar desde el estado actual?
        // PC: ¿que estoy optimizando?

        // RA: mi amount actual m (inicialmente m = amount)
        // RB: puedo:
        //      1. restarle el coins[0]
        //      ...
        //      n. restarle el coins[n]
        // RC: la menor cantidad de coins usadas

        // tengo que encontrar ma menor cantidad de coins entre
        // restar a m la cantidad de coins[0] hasta coins[n]

        if coins.len() == 1 && coins[0] < amount {
            return -1;
        }

        Solution::solve(amount, &coins)
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
