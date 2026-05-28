use colored::*;
use std::collections::HashSet;

struct Solution;

/**
 * Implementa tu solución aquí
 *
 */
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();

        for num in nums {
            if !set.contains(&num) {
                set.insert(num);
            } else {
                set.remove(&num);
            }
        }

        let ans: Vec<i32> = set.into_iter().collect();
        ans[0]
    }
}

/**
 * Pruebas unitarias
 *
 */
fn main() {
    let ans = Solution::single_number(vec![2, 2, 1]);
    println!("{}", format!("{ans}").black().bold().on_bright_green());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
        assert_eq!(Solution::single_number(vec![1]), 1);
    }
}
