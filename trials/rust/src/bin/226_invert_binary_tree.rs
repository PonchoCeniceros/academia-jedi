use colored::*;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

type Node = Option<Rc<RefCell<TreeNode>>>;

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
    pub fn pre_order(root: &Node) {
        // recorrer el arbol en preorden
        let iter: &mut Node = root;

        println!("{}", format!("{:?}", root).green().italic().underline());
    }

    // pub fn invert_tree(root: Node) -> Node {
    //     println!("{:?}", root);
    //     None
    // }
}

fn main() {
    let leaf_20: Node = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    })));

    let leaf_21: Node = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: None,
        right: None,
    })));

    let leaf_22: Node = Some(Rc::new(RefCell::new(TreeNode {
        val: 6,
        left: None,
        right: None,
    })));

    let leaf_23: Node = Some(Rc::new(RefCell::new(TreeNode {
        val: 9,
        left: None,
        right: None,
    })));

    let leaf_10: Node = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: leaf_20,
        right: leaf_21,
    })));

    let leaf_11: Node = Some(Rc::new(RefCell::new(TreeNode {
        val: 7,
        left: leaf_22,
        right: leaf_23,
    })));

    let tree_0: Node = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: leaf_10,
        right: leaf_11,
    })));

    Solution::pre_order(&tree_0);
}

/*
// Input: root = [2,1,3]
// Output: [2,3,1]
// Input: root = []
// Output: []

// Input: root = [4,2,7,1,3,6,9]
// Output: [4,7,2,9,6,3,1]
// let ans: Node = Solution::invert_tree(tree_0);
// println!("{}", format!("{:?}", ans).green().italic().underline());

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
