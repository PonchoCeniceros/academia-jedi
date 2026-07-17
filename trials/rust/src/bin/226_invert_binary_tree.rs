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
    /*
     * Recorrer un `Option<Rc<RefCell<TreeNode>>>` = atravesar 4 capas ("efecto cebolla"):
     *
     *   Option<..>   ¿hay nodo o es None?        -> abrir con `if let Some(...)`
     *   Rc<..>       propiedad compartida         -> se atraviesa sola (deref)
     *   RefCell<..>  mutabilidad interior         -> `.borrow()` (lectura) / `.borrow_mut()`
     *   TreeNode     los datos (val, left, right) -> aquí sí `.left` / `.right`
     *
     * Por eso `n.left` directo NO compila: `n` es un `&Option<..>`, no un `TreeNode`.
     * Tampoco se puede `if let Some(i) = n.left` porque intentaría MOVER un valor
     * desde atrás de un `&` (préstamo). Hay que abrir el Option prestando, y luego
     * `.borrow()` el RefCell para llegar a los campos.
     */
    fn pre_order(n: &Node) {
        if let Some(rc) = n {
            // rc: &Rc<RefCell<TreeNode>> (match ergonomics: liga por referencia)
            let node = rc.borrow(); // nodo: Ref<TreeNode> -> ya puedo leer los campos
            Solution::pre_order(&node.left); // nodo.left es Node; &nodo.left es &Node
            Solution::pre_order(&node.right);
            println!("{}", format!("{:?}", node.val).green().italic().underline()); // visitar(node.val);
        }
    }

    fn post_order(n: &Node) {
        if let Some(rc) = n {
            // rc: &Rc<RefCell<TreeNode>> (match ergonomics: liga por referencia)
            let node = rc.borrow(); // nodo: Ref<TreeNode> -> ya puedo leer los campos
            Solution::post_order(&node.left); // nodo.left es Node; &nodo.left es &Node
            Solution::post_order(&node.right);
            println!(
                "{}",
                format!("{:?}", node.val).purple().italic().underline()
            ); // visitar(node.val);
        }
    }

    fn invert_tree(n: &Node) {
        if let Some(rc) = n {
            let mut node = rc.borrow_mut();
            Solution::invert_tree(&node.left);
            Solution::invert_tree(&node.right);

            let tmp_left: Node = node.left.take();
            let tmp_right: Node = node.right.take();
            node.left = tmp_right;
            node.right = tmp_left;
        }
    }

    pub fn invert_tree_(root: Node) -> Node {
        None
    }
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
        val: 4,
        left: leaf_10,
        right: leaf_11,
    })));

    Solution::pre_order(&tree_0);
    Solution::invert_tree(&tree_0);
    Solution::post_order(&tree_0);
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
