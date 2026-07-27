use colored::*;
use katas::s;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    fn counting(i: i32) -> i32 {
        let mut x = i;
        let mut s = 0;

        while x > 0 {
            s += x % 2;
            x /= 2;
        }
        s
    }

    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        for i in 0..n + 1 {
            ans.push(Solution::counting(i));
        }
        ans
    }
}

fn main() {
    let ans = Solution::count_bits(5);
    println!("{}", format!("{:?}", ans).black().bold().on_bright_green());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_bits() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
