use colored::*;
use katas::s;

struct Solution;

/**
 * Implementa tu solución aquí
 *
 */
impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        // n.count_ones() as i32

        let mut x = n;
        let mut s = 0;
        while x > 0 {
            s += x % 2;
            x /= 2;
        }
        s
    }
}

/**
 * Pruebas unitarias
 *
 */
fn main() {
    let ans = Solution::hamming_weight(2147483645);
    println!("{}", format!("{}", ans).black().bold().on_bright_green());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_1_bits() {
        assert_eq!(Solution::hamming_weight(11), 3);
        assert_eq!(Solution::hamming_weight(128), 1);
        assert_eq!(Solution::hamming_weight(2147483645), 30);
    }
}
