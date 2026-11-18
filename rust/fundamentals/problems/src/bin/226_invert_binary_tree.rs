use colored::*;
use std::cell::RefCell;
use std::collections::VecDeque;
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

pub struct TestCase {
    label: String,
    value: Node,
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
    pub fn _pre_order(n: &Node, ans: &mut Vec<i32>) {
        if let Some(rc) = n {
            // rc: &Rc<RefCell<TreeNode>> (match ergonomics: liga por referencia)
            let node = rc.borrow(); // nodo: Ref<TreeNode> -> ya puedo leer los campos
            ans.push(node.val); // visitar(node.val)
            Solution::_pre_order(&node.left, ans); // nodo.left es Node; &nodo.left es &Node
            Solution::_pre_order(&node.right, ans);
        }
    }

    pub fn bfs(root: &Node) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut queue = VecDeque::new();
        // Si la raíz existe, la metemos a la queue clonando el Rc
        if let Some(rc) = root {
            queue.push_back(Rc::clone(rc));
        }
        // Mientras queden nodos por procesar en este nivel o los siguientes
        while let Some(current_rc) = queue.pop_front() {
            // Accedemos al nodo de forma inmutable
            let node = current_rc.borrow();
            // Guardamos el valor en nuestro arreglo de salida
            ans.push(node.val);
            // Empujamos los hijos a la queue (si existen)
            if let Some(ref left) = node.left {
                queue.push_back(Rc::clone(left));
            }
            if let Some(ref right) = node.right {
                queue.push_back(Rc::clone(right));
            }
        }

        ans
    }

    fn _invert_tree(n: &Node) {
        if let Some(rc) = n {
            let mut node = rc.borrow_mut();
            Solution::_invert_tree(&node.left);
            Solution::_invert_tree(&node.right);

            let tmp_left: Node = node.left.take();
            let tmp_right: Node = node.right.take();
            node.left = tmp_right;
            node.right = tmp_left;
        }
    }

    pub fn invert_tree(root: Node) -> Node {
        Solution::_invert_tree(&root);
        root
    }
}

fn main() {
    let tree: Node = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    })));

    let tree_ans: Node = Solution::invert_tree(tree);
    let ans: Vec<i32> = Solution::bfs(&tree_ans);
    println!("{}", format!("{:?}", ans).green().italic().underline());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invert_binary_tree() {
        //
        //
        //
        let leaf_x: Node = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));

        let leaf_y: Node = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));

        let tree_1: Node = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: leaf_x,
            right: leaf_y,
        })));

        //
        //
        //
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

        let cases = [
            (
                TestCase {
                    label: "tree_0".to_string(),
                    value: tree_0,
                },
                vec![4, 7, 2, 9, 6, 3, 1],
            ),
            (
                TestCase {
                    label: "tree_1".to_string(),
                    value: tree_1,
                },
                vec![2, 3, 1],
            ),
        ];

        for (input, expected) in cases {
            let label = input.label;
            let tree_ans: Node = Solution::invert_tree(input.value);
            let ans: Vec<i32> = Solution::bfs(&tree_ans);
            assert_eq!(ans, expected, "{}", label.red().italic().underline());
        }
    }
}
