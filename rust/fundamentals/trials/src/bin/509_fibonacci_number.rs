use std::collections::HashMap;

use colored::*;
use katas::s;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn compute(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if n < 2 {
            return n;
        }

        // Si el resultado de memo.get(&n) coincide con el patrón Some,
        // abre la caja, toma el valor al que apunta la referencia (&)
        // y guárdalo en una nueva variable llamada resultado
        if let Some(&ans) = memo.get(&n) {
            return ans;
        }

        let n1 = Solution::compute(n - 1, memo);
        let n2 = Solution::compute(n - 2, memo);

        memo.insert(n, n1 + n2);
        n1 + n2
    }

    pub fn fib(n: i32) -> i32 {
        let mut memo: HashMap<i32, i32> = HashMap::new();
        Solution::compute(n, &mut memo)
    }
}

fn main() {
    let ans = Solution::fib(2);
    println!("{}", format!("{}", ans).black().bold().on_bright_green());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_number() {
        assert_eq!(Solution::fib(2), 1);
        assert_eq!(Solution::fib(3), 2);
        assert_eq!(Solution::fib(4), 3);
        assert_eq!(Solution::fib(0), 0);
        assert_eq!(Solution::fib(1), 1);
    }
}
