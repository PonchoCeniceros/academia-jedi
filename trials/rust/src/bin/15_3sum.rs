use colored::*;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    /**
     *
     */
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        for k in 0..nums.len() {
            // @todo condicionar los valores de los indices segun el valor de k
            let (mut i, mut j) = (0_usize, nums.len() - 1);
            let x = nums[k];

            while j > i {
                let (y, z) = (nums[i], nums[j]);
                let s = y + z;
                if s == -x {
                    println!("(x,y,z) = ({},{},{})", x, y, z);
                    break;
                }

                if s > -x {
                    j = if j == k {
                        (j as i32 - 2) as usize
                    } else {
                        (j as i32 - 1) as usize
                    };
                }

                if s < -x {
                    i = if i == k { i + 2 } else { i + 1 };
                }
            }
        }

        vec![vec![0]]
    }
}

fn main() {
    // let nums = vec![-1, 0, 1, 2, -1, -4];
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
