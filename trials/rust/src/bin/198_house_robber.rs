use colored::*;
use std::cmp::max;

struct Solution;

/**
 * Implement your solution here
 */
impl Solution {
    // me encuentro en la casa curr (y tengo la adjacencia prev)
    fn solve(curr: usize, prev: usize, houses: &[i32]) -> i32 {
        // condicion base: si se termina el arreglo de casa que robar, terminamos
        if curr >= houses.len() {
            return 0;
        }

        // puedo (1) robar la casa adjacente o (2) brincarme esta casa
        // y pasarme a la siguiente de la adjacente
        //
        // dependiendo de la adjacencia puedo o no tomar el botin de la casa curr
        let loot = if curr == prev + 1 { 0 } else { houses[curr] };

        // ¿cual de las 2 decisiones me deja con mas dinero?
        loot + max(
            Solution::solve(curr + 1, curr, houses),
            Solution::solve(curr + 2, curr, houses),
        )
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        // ¿cual de las 2 decisiones me deja con mas dinero?
        max(
            Solution::solve(0, 0, &nums), // empezar por robar la primer casa
            Solution::solve(1, 1, &nums), // empezar por robar desde la siguiente casa
        )
    }
}

fn main() {
    let ans = Solution::rob(vec![1, 2]);
    println!("{}", format!("{}", ans).green().italic().underline());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_house_robber() {
        let cases = [
            (vec![1, 2], 2),
            (vec![2, 1, 1, 2], 4),
            (vec![1, 2, 3, 1], 4),
            (vec![2, 7, 9, 3, 1], 12),
        ];

        for (input, expected) in cases {
            assert_eq!(
                Solution::rob(input.clone()),
                expected,
                "{}",
                format!("{:?}", input).red().italic().underline()
            );
        }
    }
}
