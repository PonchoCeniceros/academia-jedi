use colored::*;
use katas::s;

struct Solution;

/**
 * Implementa tu solución aquí
 *
 */
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let x_str = x.to_string();
        let x_arr = x_str.as_bytes();
        let x_len = x_arr.len();

        for i in 0..x_len {
            // println!(
            //     "{} {}",
            //     format!("{}", x_arr[i]).bold().on_red(),
            //     format!("{}", x_arr[x_len - 1 - i]).bold().on_blue(),
            // );
            if x_arr[i] != x_arr[x_len - 1 - i] {
                return false;
            }
        }
        true
    }
}

/**
 * Pruebas unitarias
 *
 */
fn main() {
    let ans = Solution::is_palindrome(121);
    println!("{}", format!("{ans}").black().bold().on_bright_green());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_number() {
        assert!(Solution::is_palindrome(121));
        assert!(!Solution::is_palindrome(-121));
        assert!(!Solution::is_palindrome(10));
    }
}
