use colored::*;
use katas::s;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // acumular la frecuencia en un map {num, freq},
        // ordenar respecto a las frecuencias,
        // extraer los nums y entregar un slice hasta el numero pedido
        vec![0]
    }
}

fn main() {
    let k = 2;
    let nums = vec![1, 1, 1, 2, 2, 3];
    let ans = Solution::top_k_frequent(nums, k);
    println!("{:?}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent_elements() {
        let cases = [
            (vec![1, 1, 1, 2, 2, 3], 2, vec![1, 2]),
            (vec![1], 1, vec![1]),
            (vec![1, 2, 1, 2, 1, 2, 3, 1, 3, 2], 2, vec![1, 2]),
        ];

        for (in1, in2, expected) in cases {
            assert_eq!(
                Solution::top_k_frequent(in1.clone(), in2),
                expected,
                "{}",
                format!("{:?}, {}", in1, in2).red().italic().underline()
            );
        }
    }
}
