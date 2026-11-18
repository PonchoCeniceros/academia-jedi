use colored::*;
use katas::s;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn find_k_or(nums: Vec<i32>, k: i32) -> i32 {
        let mut answ = 0;
        let mut mask = 1;

        for _ in 0..32 {
            let mut cnt = 0;
            for n in nums.iter() {
                let bit = n & mask;
                cnt = if bit > 0 { cnt + 1 } else { cnt }
            }

            if cnt >= k {
                answ += mask;
            }

            mask <<= 1;
        }

        answ
    }
}

fn main() {
    let _ans = Solution::find_k_or(vec![7, 12, 9, 8, 9, 15], 4);
    println!(
        "{}",
        "May the Force be with you!"
            .black()
            .bold()
            .on_bright_green()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_the_kor_of_an_array() {
        assert_eq!(Solution::find_k_or(vec![7, 12, 9, 8, 9, 15], 4), 9);
        assert_eq!(Solution::find_k_or(vec![2, 12, 1, 11, 4, 5], 6), 0);
        assert_eq!(Solution::find_k_or(vec![10, 8, 5, 9, 11, 6, 8], 1), 15);
    }
}
