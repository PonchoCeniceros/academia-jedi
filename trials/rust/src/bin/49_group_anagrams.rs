use colored::*;
use katas::s;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        vec![vec!["".to_string()]]
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
        let cases = [
            (
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
            ),
            // (, ),
        ];

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
