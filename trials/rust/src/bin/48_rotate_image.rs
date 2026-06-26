struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        todo!();
    }
}

fn main() {
    //
    let mut matrix_1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate(&mut matrix_1);
    //
    let mut matrix_2 = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    Solution::rotate(&mut matrix_2);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_rotate_image() {
        todo!();
    }
}
