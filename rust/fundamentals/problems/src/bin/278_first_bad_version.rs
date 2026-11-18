use colored::*;

struct Solution;

/**
 * Implement your solution here
 */
impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut l = 0;
        let mut r = n;

        while l <= r {
            let m = l + (r - l) / 2;
            if self.isBadVersion(m) {
                r = m - 1
            } else {
                l = m + 1
            }
        }

        l
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
