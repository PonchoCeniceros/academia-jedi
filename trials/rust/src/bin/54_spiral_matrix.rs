use colored::*;

struct Solution;

type i32_matrix = Vec<Vec<i32>>;

/**
 * Implement your solution here
 *
 */
impl Solution {
    fn next(i: usize, j: usize, direction: char) -> (usize, usize) {
        match direction {
            'u' => (i - 1, j),
            'd' => (i + 1, j),
            'l' => (i, j - 1),
            'r' => (i, j + 1),
            _ => (i, j), // Caso por defecto: si es un espacio u otro carácter, no se mueve
        }
    }

    fn turn(matrix: i32_matrix, i: usize, j: usize, direction: char) -> char {
        let directions = vec![
            ('u', (i - 1, j)),
            ('d', (i + 1, j)),
            ('l', (i, j - 1)),
            ('r', (i, j + 1)),
        ];

        for (dir, coords) in directions.iter() {
            if let Some(&val) = matrix.get(coords.0)?.get(coords.1) {
                println!("turn to {}", dir);
            }
        }
    }

    pub fn spiral_order(matrix: i32_matrix) -> Vec<i32> {
        let mut dir: char = 'r';
        let mut ans: Vec<i32> = vec![];
        let (mut c, mut n) = (0, 0);
        let (mut i, mut j) = (0_usize, 0_usize);

        // primer valor capturado
        ans.push(matrix[i][j]);

        loop {
            // incrementar contador general
            // incrementar contador por linea
            c += 1;

            if c == 4 {
                break;
            }
        }

        ans
    }
}

fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let ans = Solution::spiral_order(matrix);
    println!("{:?}", format!("{:?}", ans).green().italic().underline());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_matrix() {
        let cases = [
            (
                vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
                vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
            ),
            (
                vec![
                    vec![1, 2, 3, 4],
                    vec![5, 6, 7, 8],
                    vec![9, 10, 11, 12],
                    vec![13, 14, 15, 16],
                ],
                vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
            ),
        ];

        for (input, expected) in cases {
            assert_eq!(
                Solution::spiral_order(input.clone()),
                expected,
                "{}",
                format!("{:?}", input).red().italic().underline()
            );
        }
    }
}
