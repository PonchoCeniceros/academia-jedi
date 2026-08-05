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
        println!("{:?} {}", nums, target);

        if nums.len() == 1 && target == nums[0] {
            return 0;
        }

        let mut l = 0;
        let mut r = nums.len() as i32 - 1;

        while l <= r {
            let m = l + (r - l) / 2;

            if target == nums[m as usize] {
                return m + delta;
            }

            if target < nums[m as usize] {
                r = m - 1;
            }

            if target > nums[m as usize] {
                l = m + 1;
            }
        }

        -1
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // #1 primero encontrar el cero (el desplazamiento)
        let delta = Solution::get_delta(&nums);

        println!("δ = {}", format!("{}", delta).purple().italic().underline());

        // #1.5 si no hay dislocamiento, hacer BS en todo el arr
        // if delta == 0 {
        //     return Solution::make_bs(&nums, target, 0_i32);
        // }

        // #2 luego hacer la BS considerando el desplazamiento
        let l = 0_i32;
        let s = delta.saturating_sub(1);
        let t = delta;
        let r = nums.len() as i32;

        let nums_izq = &nums[(l as usize)..s.saturating_add(1)];
        let nums_der = &nums[t..r as usize];

        println!(
            "{}{}",
            "(l, s, t, r) = ".purple().italic().underline(),
            format!("({}, {}, {}, {})", l, s, t, r)
        );

        println!(
            "nₗₛ = {}",
            format!("{:?}", nums_izq).cyan().italic().underline()
        );
        println!(
            "nₜᵣ = {}",
            format!("{:?}", nums_der).blue().italic().underline()
        );

        println!(
            "τ ∈ [{},{}] | τ ∈ [{},{}]",
            nums[l as usize],
            nums[s],
            nums[t],
            nums[(r - 1) as usize]
        );

        // nums[l as usize] <= target && target <= nums[s],
        // nums[t] <= target && target <= nums[(r - 1) as usize]

        if nums[l as usize] <= target && target <= nums[s] {
            println!("slice izquierdo");
            // slice izquierdo
            Solution::make_bs(nums_izq, target, 0_i32)
        } else {
            println!("slice derecho");
            // slice derecho
            Solution::make_bs(nums_der, target, delta as i32)
        }

        // if nums[t] <= target && target <= nums[(r - 1) as usize] {
        //     println!("slice derecho");
        //     // slice derecho
        //     Solution::make_bs(&nums[t..r as usize], target, delta as i32)
        // } else {
        //     println!("slice izquierdo");
        //     // slice izquierdo
        //     Solution::make_bs(&nums[(l as usize)..s], target, 0_i32)
        // }
    }
}

fn get_test_cases() -> Vec<((Vec<i32>, i32), i32)> {
    vec![
        ((vec![5, 1, 3], 5), 0),
        ((vec![3, 5, 1], 3), 0),
        ((vec![3, 1], 3), 0),
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
