use katas::s;
use std::collections::HashSet;
struct Solution;

/**
 * Implementa tu solución aquí
 *
 */
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hash_table: HashSet<i32> = HashSet::new();
        for n in nums.iter() {
            if hash_table.contains(&n) {
                return true;
            }
            hash_table.insert(*n);
        }
        false
    }
}

/**
 * Pruebas unitarias
 *
 */
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
        assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
        assert!(Solution::contains_duplicate(vec![
            1, 1, 1, 3, 3, 4, 3, 2, 4, 2
        ]));
    }
}
