use colored::*;

struct Solution;

/**
 * Implement your solution here
 */
impl Solution {
    fn get_delta(nums: &[i32]) -> usize {
        let mut l: i32 = 0;
        let mut r = nums.len() as i32 - 1;

        while l <= r {
            if nums[l as usize] == nums[r as usize] {
                return l as usize;
            }

            let m = l + (r - l) / 2;

            if nums[m as usize] < nums[r as usize] {
                r = m - 1
            } else {
                l = m + 1
            }
        }

        l as usize
    }

    fn make_bs(nums: &[i32], target: i32, delta: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            let idx = mid as usize;

            if target == nums[idx] {
                return mid + delta;
            }

            if target < nums[idx] {
                right = mid - 1;
            }

            if target > nums[idx] {
                left = mid + 1;
            }
        }

        -1
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // #1 primero encontrar el cero (el desplazamiento)
        let delta = Solution::get_delta(&nums);

        // #2 luego hacer la BS considerando el desplazamiento
        let l = 0_i32;
        let s = delta.saturating_sub(1);
        let t = delta;
        let r = nums.len() as i32;

        if target <= nums[0] {
            // slice derecho
            Solution::make_bs(&nums[t..r as usize], target, delta as i32)
        } else {
            // slice izquierdo
            Solution::make_bs(&nums[(l as usize)..s], target, 0_i32)
        }
    }
}

fn get_test_cases() -> Vec<((Vec<i32>, i32), i32)> {
    vec![
        ((vec![1, 3], 3), 1),
        ((vec![6, 7, 0, 1, 2, 4, 5], 0), 2),
        ((vec![4, 5, 6, 7, 0, 1, 2], 0), 4),
        ((vec![4, 5, 6, 7, 0, 1, 2], 3), -1),
        ((vec![1], 0), -1),
    ]
}

fn main() {
    let cases = get_test_cases();
    let (input, _expected) = &cases[0];
    let ans = Solution::search(input.0.clone(), input.1);
    println!("{}", format!("{}", ans).green().italic().underline());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_in_rotated_sorted_array() {
        let cases = get_test_cases();

        for (input, expected) in cases {
            assert_eq!(
                Solution::search(input.0.clone(), input.1),
                expected,
                "{}",
                format!("{:?}", input).red().italic().underline()
            );
        }
    }
}
