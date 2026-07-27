use colored::*;
use katas::s;
use std::collections::HashSet;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();
        nums.retain(|&x| set.insert(x));
        nums.len() as i32
    }
}

fn main() {
    let mut arr = vec![1, 1, 2];
    let ans = Solution::remove_duplicates(&mut arr);
    println!(
        "{}: {}",
        format!("{}", ans).black().bold().on_bright_green(),
        format!("{:?}", arr).black().bold().on_bright_cyan()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates_from_sorted_array() {
        //
        // test 1
        //
        let exp = [1, 2];
        let mut arr = vec![1, 1, 2];
        let ans = Solution::remove_duplicates(&mut arr);

        assert_eq!(ans, exp.len() as i32);
        for idx in 0..ans {
            assert_eq!(arr[idx as usize], exp[idx as usize]);
        }

        //
        // test 2
        //
        let exp = [0, 1, 2, 3, 4];
        let mut arr = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let ans = Solution::remove_duplicates(&mut arr);

        assert_eq!(ans, exp.len() as i32);
        for idx in 0..ans {
            assert_eq!(arr[idx as usize], exp[idx as usize]);
        }
    }
}
