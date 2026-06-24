use colored::*;
use std::collections::HashSet;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut set: HashSet<Vec<i32>> = HashSet::new();
        let mut arr = nums.clone();
        arr.sort_by(|a, b| b.cmp(a));

        for k in 0..arr.len() {
            let mut i: usize = if k == 0 { 1 } else { 0 };
            let mut j: usize = if k == arr.len() - 1 {
                arr.len() - 2
            } else {
                arr.len() - 1
            };

            while j > i {
                let (x, y, z) = (arr[k], arr[i], arr[j]);
                let s = x + y + z;
                if s == 0 {
                    let mut ans = vec![x, y, z];
                    ans.sort();
                    set.insert(ans);
                    // println!("(x,y,z) = ({},{},{})", x, y, z);
                    break;
                }

                if s < 0 {
                    j = if j.saturating_sub(1) == k {
                        j.saturating_sub(2) // evitando el casteo (j as i32 - 2) as usize
                    } else {
                        j.saturating_sub(1)
                    };
                }

                if s > 0 {
                    i = if i + 1 == k { i + 2 } else { i + 1 };
                }
            }
        }

        let mut ans: Vec<Vec<i32>> = set.into_iter().collect();
        ans.sort();
        ans
    }
}

fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let ans = Solution::three_sum(nums);
    println!("{}", format!("{:?}", ans).green().italic().underline());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        let cases: [(Vec<i32>, Vec<Vec<i32>>); 4] = [
            (vec![0, 1, 1], vec![]),
            (vec![0, 0, 0], vec![vec![0, 0, 0]]),
            (
                vec![-1, 0, 1, 2, -1, -4],
                vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            ),
            (
                vec![2, -3, 0, -2, -5, -5, -4, 1, 2, -2, 2, 0, 2, -4, 5, 5, -10],
                vec![
                    vec![-10, 5, 5],
                    vec![-5, 0, 5],
                    vec![-4, 2, 2],
                    vec![-3, -2, 5],
                    vec![-3, 1, 2],
                    vec![-2, 0, 2],
                ],
            ),
        ];

        for (input, expected) in cases {
            assert_eq!(
                Solution::three_sum(input.clone()),
                expected,
                "{}",
                format!("{:?}", input).red().italic().underline()
            );
        }
    }
}
