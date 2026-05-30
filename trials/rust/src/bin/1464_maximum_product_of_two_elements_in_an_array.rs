use colored::*;
use katas::s;

struct Solution;

/**
 * Implementa tu solución aquí
 *
 */
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut used_idx = 0_usize;
        let mut val_u = 0;
        let mut val_v = 0;

        for (idx, val) in nums.iter().enumerate() {
            if *val > val_u {
                val_u = *val;
                used_idx = idx;
            }
        }

        for (idx, val) in nums.iter().enumerate() {
            if used_idx != idx && *val > val_v {
                val_v = *val;
            }
        }

        (val_u - 1) * (val_v - 1)
    }
}

/**
 * Pruebas unitarias
 *
 */
fn main() {
    println!("{}", "Hello World!".black().bold().on_bright_green());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_product_of_two_elements_in_an_array() {
        assert_eq!(Solution::max_product(vec![3, 4, 5, 2]), 12);
        assert_eq!(Solution::max_product(vec![1, 5, 4, 5]), 16);
        assert_eq!(Solution::max_product(vec![3, 7]), 12);
    }
}
