use std::collections::HashMap;
struct Solution;

/**
 * Implementa tu solución aquí
 *
 */
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_table: HashMap<i32, i32> = HashMap::new();
        let mut x: i32 = 0;
        let mut y: i32 = 0;

        for (idx, val) in nums.iter().enumerate() {
            y = target - val;

            if let Some(&ans) = hash_table.get(&y) {
                x = ans;
                y = idx as i32;
                return vec![x, y];
            }
            // es quivalente a:
            //
            // let ans = hash_table.get(&y);
            // if ans != None {
            //     x = *ans.unwrap();
            //     y = idx as i32;
            //     return vec![x, y];
            // }
            //
            hash_table.insert(*val, idx as i32);
        }
        vec![x, y]
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
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }
}
