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

        if n == 2 {
            return 1;
        }

        // Si el resultado de memo.get(&n) coincide con el patrón Some,
        // abre la caja, toma el valor al que apunta la referencia (&)
        // y guárdalo en una nueva variable llamada resultado
        if let Some(&ans) = memo.get(&n) {
            return ans;
        }

        let n1 = Solution::compute(n - 1, memo);
        let n2 = Solution::compute(n - 2, memo);
        let n3 = Solution::compute(n - 3, memo);

        memo.insert(n, n1 + n2 + n3);
        n1 + n2 + n3
    }

    pub fn tribonacci(n: i32) -> i32 {
        let mut memo: HashMap<i32, i32> = HashMap::new();
        Solution::compute(n, &mut memo)
    }
}

fn main() {
    let ans = Solution::tribonacci(4);
    println!("{}", format!("{}", ans).black().bold().on_bright_green());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_tribonacci_number() {
        assert_eq!(Solution::tribonacci(4), 4);
        assert_eq!(Solution::tribonacci(25), 1389537);
        assert_eq!(Solution::tribonacci(0), 0);
        assert_eq!(Solution::tribonacci(1), 1);
        assert_eq!(Solution::tribonacci(2), 1);
    }
}
