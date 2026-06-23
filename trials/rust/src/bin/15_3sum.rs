use colored::*;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    fn delta_0(x: usize, y: usize, z: usize, w: usize, n: &[i32], ans: &mut Vec<Vec<i32>>) {
        if n[x] + n[y] + n[z] == 0 {
            //  - n[w]
            ans.push(vec![x as i32, y as i32, z as i32]);
        }
    }

    fn delta_1(x: usize, y: usize, z: usize, n: &[i32], ans: &mut Vec<Vec<i32>>) {
        if n[x] + n[y] + n[z] == 0 {
            ans.push(vec![x as i32, y as i32, z as i32]);
        }
    }

    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let i = 3;
        let mut ans: Vec<Vec<i32>> = Vec::new();

        for j in 0..nums.len() - 1 {
            if j == i {
                println!("--");
                continue;
            }

            if j == 0 {
                if j + 1 == i {
                    println!("δ₁({},{},{})", i, j, j + 2);
                    Solution::delta_1(i, j, j + 2, &nums, &mut ans);
                } else {
                    println!("δ₁({},{},{})", i, j, j + 1);
                    Solution::delta_1(i, j, j + 1, &nums, &mut ans);
                }
                continue;
            }

            if j == i + 1 {
                println!("δ₀({},{},{}, {})", i, j, j as i32 - 2, j + 1);
                Solution::delta_0(i, j, (j as i32 - 2) as usize, j + 1, &nums, &mut ans);
                continue;
            }

            if j + 1 == i {
                println!("δ₀({},{},{}, {})", i, j, j + 2, j as i32 - 1);
                Solution::delta_0(i, j, j + 2, (j as i32 - 1) as usize, &nums, &mut ans);
                continue;
            }

            println!("δ₀({},{},{}, {})", i, j, j + 1, j as i32 - 1);
            Solution::delta_0(i, j, j + 1, (j as i32 - 1) as usize, &nums, &mut ans);
        }

        ans
        // vec![vec![0]]
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
        let cases: [(Vec<i32>, Vec<Vec<i32>>); 3] = [
            (vec![0, 1, 1], vec![]),
            (vec![0, 0, 0], vec![vec![0, 0, 0]]),
            (
                vec![-1, 0, 1, 2, -1, -4],
                vec![vec![-1, -1, 2], vec![-1, 0, 1]],
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

/*
if sigma_0:
   if sigma_3:
       delta_1(i, j, j+2)
   else:
       delta_1(i, j, j+1)

if sigma_1:
    continue

if sigma_2:
       delta_0(i, j, j+1, j-2)

if sigma_3:
       delta_0(i, j, j+2, j-1)

*/
