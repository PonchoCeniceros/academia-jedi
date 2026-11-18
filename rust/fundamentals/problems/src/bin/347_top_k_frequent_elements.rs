use colored::*;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

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

        let mut set: HashMap<i32, i32> = HashMap::new();

        for num in nums.into_iter() {
            match set.entry(num) {
                Vacant(e) => {
                    e.insert(1);
                }
                Occupied(mut e) => {
                    *e.get_mut() += 1; // Obtenemos la referencia mutable y le sumamos 1
                }
            }
        }

        let mut el: Vec<(i32, i32)> = set.into_iter().collect();
        el.sort_by(|a, b| b.1.cmp(&a.1));
        let ans: Vec<i32> = el.into_iter().take(k as usize).map(|(k, _v)| k).collect();

        ans
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
