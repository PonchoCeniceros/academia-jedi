use colored::*;
use std::cmp::max;
use std::collections::HashMap;

struct Solution;

/**
 * Implement your solution here
 */
impl Solution {
    // me encuentro en la casa curr
    fn solve(curr: usize, houses: &[i32], memo: &mut HashMap<usize, i32>) -> i32 {
        // condicion base: si se termina el arreglo de casa que robar, terminamos
        if curr >= houses.len() {
            return 0;
        }

        /* MEMOIZACION */
        if let Some(&ans) = memo.get(&curr) {
            return ans;
        }

        // tengo estas 2 decisiones que puedo tomar:
        // () robar la casa actual (y tener que brincarme la sig casa porque no podre robarle)
        // () brincerme la casa actual (y no tomar el botin)
        let steal = houses[curr] + Solution::solve(curr + 2, houses, memo);
        let skip = Solution::solve(curr + 1, houses, memo);
        let ans = max(steal, skip);
        /* MEMOIZACION */
        memo.insert(curr, ans);
        ans
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut memo: HashMap<usize, i32> = HashMap::new();
        Solution::solve(0, &nums, &mut memo)
    }
}

fn main() {
    let ans = Solution::rob(vec![1, 2]);
    println!("{}", format!("{}", ans).green().italic().underline());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_house_robber() {
        let cases = [
            (vec![1, 2], 2),
            (vec![2, 1, 1, 2], 4),
            (vec![1, 2, 3, 1], 4),
            (vec![2, 7, 9, 3, 1], 12),
        ];

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
