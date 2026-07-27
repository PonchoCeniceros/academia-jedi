struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    fn compute(s: usize, n: usize, matrix: &mut Vec<Vec<i32>>) {
        if n < 2 {
            return;
        }

        for p in 0..(n - 1) {
            let (mut i, mut j) = (0, p);
            let (mut l, mut m) = (0_usize, 0_usize);
            let (mut prev, mut aux) = (0, 0);

            loop {
                let cond = (i, j) == (0, p);
                prev = if cond { matrix[0 + s][p + s] } else { aux };
                (l, m) = (j, n - 1 - i);
                aux = matrix[l + s][m + s];
                matrix[l + s][m + s] = prev;
                (i, j) = (l, m);

                if (i, j) == (0, p) {
                    break;
                }
            }
        }
    }

    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut n = matrix[0].len();
        let mut s = 0_usize;

        loop {
            if n < 2 {
                break;
            }

            Solution::compute(s, n, matrix);

            n = n.saturating_sub(2);
            s += 1;
        }
    }
}

fn main() {
    let mut matrix_0 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate(&mut matrix_0);
    println!("{:?}", matrix_0);

    //
    let mut matrix_1 = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ];
    Solution::rotate(&mut matrix_1);
    println!("{:?}", matrix_1);

    //
    let mut matrix_2 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate(&mut matrix_2);
    println!("{:?}", matrix_2);

    //
    let mut matrix_3 = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    Solution::rotate(&mut matrix_3);
    println!("{:?}", matrix_3);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_rotate_image() {
        todo!();
    }
}
