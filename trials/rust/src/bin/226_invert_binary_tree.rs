use colored::*;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

/**
 * Definition for a binary tree node
 */
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

/**
 * Implement your solution here
 */
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        None
    }
}

fn main() {
    // Input: root = [4,2,7,1,3,6,9]
    // Output: [4,7,2,9,6,3,1]
    // Input: root = [2,1,3]
    // Output: [2,3,1]
    // Input: root = []
    // Output: []
    let ans = Solution::invert_tree(None);
    println!("{}", format!("{:?}", ans).green().italic().underline());
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invert_binary_tree() {
        let cases = [
            // (, ),
        ];

        for (input, expected) in cases {
            todo!();
            // assert_eq!(
            //     Solution::invert_binary_tree(),
            //     expected,
            //     "{}",
            //     format!("{:?}", input).red().italic().underline()
            // );
        }
    }
}
*/
