use colored::*;
use katas::s;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    /**
     * ordenar lexicograficamente un string dado
     */
    fn sort_string(src: String) -> String {
        let mut aux: Vec<char> = src.chars().collect();
        aux.sort();
        aux.into_iter().collect()
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut set: HashMap<String, Vec<String>> = HashMap::new();
        let mut ans: Vec<Vec<String>> = Vec::new();

        for s in strs.into_iter() {
            let k = Solution::sort_string(s.clone());

            match set.entry(k) {
                Vacant(e) => {
                    e.insert(vec![s]);
                }
                Occupied(mut e) => {
                    e.get_mut().push(s);
                }
            }
        }

        for c in set.values() {
            ans.push(c.clone());
        }

        ans
    }
}

fn main() {
    let strs: Vec<String> = vec![
        s!("eat"),
        s!("tea"),
        s!("tan"),
        s!("ate"),
        s!("nat"),
        s!("bat"),
    ];
    let ans = Solution::group_anagrams(strs);
    println!("{:?}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagrams() {
        let cases = [(
            vec![
                s!("eat"),
                s!("tea"),
                s!("tan"),
                s!("ate"),
                s!("nat"),
                s!("bat"),
            ],
            vec![
                vec![s!("bat")],
                vec![s!("nat"), s!("tan")],
                vec![s!("ate"), s!("eat"), s!("tea")],
            ],
        )];

        for (input, expected) in cases {
            assert_eq!(
                Solution::group_anagrams(input.clone()),
                expected,
                "{}",
                format!("{:?}", input).red().italic().underline()
            );
        }
    }
}
