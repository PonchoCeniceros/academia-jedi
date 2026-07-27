use colored::*;

struct Solution;

/**
 * Implement your solution here
 *
 */
impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        0
    }
}

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)
impl Solution {
    fn isBadVersion(&self, version: i32) -> bool {
        version >= 4
    }
}

fn main() {
    let ans = Solution::first_bad_version(&Solution, 5);
    println!("{}", format!("{}", ans).green().italic().underline());
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_bad_version() {
        let cases = [
            (5, 4), // bad = 4
            (1, 1), // bad = 1
        ];

        for (input, expected) in cases {
            assert_eq!(
                Solution::first_bad_version(input),
                expected,
                "{}",
                format!("{:?}", input).red().italic().underline()
            );
        }
    }
}
*/
