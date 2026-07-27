use colored::*;
use katas::s;

use regex::Regex;
use std::collections::HashSet;
struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn solver(e: &[i32], ops: &[u8]) -> Vec<i32> {
        if e.len() == 1 {
            return Vec::from(e);
        }

        let mut arr: Vec<i32> = vec![];
        for (i, op) in ops.iter().enumerate() {
            for x in Solution::solver(&e[0..i + 1], &ops[0..i]) {
                for y in Solution::solver(&e[(i + 1)..e.len()], &ops[(i + 1)..ops.len()]) {
                    let ans = match *op {
                        43_u8 => x + y,
                        45_u8 => x - y,
                        _ => x * y,
                    };
                    arr.push(ans);
                }
            }
        }
        arr
    }

    fn tokenize(expr: &str) -> Vec<u8> {
        let mut tokens = Vec::new();
        let mut current = String::new();

        for c in expr.chars() {
            if c.is_ascii_digit() {
                current.push(c);
            } else if !current.is_empty() {
                tokens.push(current.parse::<u8>().unwrap());
                current.clear();
            }
        }

        if !current.is_empty() {
            tokens.push(current.parse::<u8>().unwrap());
        }

        tokens
    }

    pub fn get_ops(expression: String) -> (Vec<i32>, Vec<u8>) {
        const ZERO_ASCII: u8 = 48;
        let ops_set: HashSet<u8> = HashSet::from([43, 45, 42]); // ["+", "-", "*"]
        let tokens = expression.as_bytes();
        let mut e: Vec<i32> = vec![];
        let mut ops: Vec<u8> = vec![];

        for t in tokens.iter() {
            if ops_set.contains(t) {
                ops.push(*t);
            } else {
                e.push((*t - ZERO_ASCII) as i32);
            }
        }
        (e, ops)
    }

    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let (e, ops) = Solution::get_ops(expression);

        println!("{}", format!("{:?}", e).black().bold().on_bright_red());
        println!("{}", format!("{:?}", ops).black().bold().on_bright_yellow());

        if e.len() == 1 {
            return e;
        }

        Solution::solver(&e, &ops)
    }
}

fn main() {
    let expression = s!("2-11-1");
    let re = Regex::new(r"(?:[1-9][0-9]|[0-9])|[+\-*]").unwrap();

    let tokens = re.find_iter(&expression).map(|m| m.as_str()).collect();

    let ans = Solution::diff_ways_to_compute(s!("11"));
    println!("{}", format!("{:?}", ans).black().bold().on_bright_green());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_different_ways_to_add_parentheses() {
        let ans_1: Vec<i32> = vec![11];
        let ans_2: Vec<i32> = vec![0, 2];
        let ans_3: Vec<i32> = vec![-34, -14, -10, -10, 10];
        assert_eq!(Solution::diff_ways_to_compute(s!("11")), ans_1);
        assert_eq!(Solution::diff_ways_to_compute(s!("2-1-1")), ans_2);
        assert_eq!(Solution::diff_ways_to_compute(s!("2*3-4*5")), ans_3);
    }
}
