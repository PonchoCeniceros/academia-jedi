use colored::*;

type Node = Option<Box<ListNode>>;

/**
 * definicion del nodo de una lista enlazada
 */
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

/**
 * implementacion del nodo de una lista enlazada
 */
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/**
 * objeto solucion
 */
struct Solution;

/**
 * implementacion de la solucion
 */
impl Solution {
    fn get_vec(head: Node) -> Vec<i32> {
        // iterador apuntando a la cabeza de la lista enlazada
        let mut iter = &head;

        while let Some(node) = iter {
            println!("{}", node.val);
            iter = &node.next;
        }

        vec![0]
    }

    fn build(values: Vec<i32>) -> Node {
        // Inicializas la cabeza de la lista enlazada
        let mut head: Node = None;
        // iter es simplemente un puntero láser (una referencia mutable) que está apuntando directamente a head
        let mut iter: &mut Node = &mut head;

        for &val in values.iter() {
            // Para asignarle un valor real a head a través de iter,
            // debes modificar el contenido de donde apunta iter, no la referencia en sí.
            *iter = Some(Box::new(ListNode { val, next: None }));
            // Para avanzar iter al siguiente nodo de forma segura:
            // .as_mut() transforma el &mut Option<Box<ListNode>> en un Option<&mut Box<ListNode>>
            // .unwrap() ahora nos da el &mut Box<ListNode> seguro (sin mover el valor)
            iter = &mut iter.as_mut().unwrap().next;
        }

        head
    }

    pub fn reverse_list(head: Node) -> Node {
        println!("{:?}", head);
        None
    }
}

fn main() {
    let ll0: Node = Solution::build(vec![1, 2, 3, 4, 5]);
    println!("{}", format!("{:?}", ll0).purple().italic().underline());
    let _ = Solution::get_vec(ll0);
    // let ans: Option<Box<ListNode>> = Solution::reverse_list(ll0);
    // println!("{}", format!("{:?}", ans).green().italic().underline());
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_linked_list() {
        let cases = [
            (vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]),
            (vec![1, 2], vec![2, 1]),
            (vec![], vec![]),
        ];

        for (input, expected) in cases {
            todo!();
            // assert_eq!(
            //     Solution::reverse_linked_list(),
            //     expected,
            //     "{}",
            //     format!("{:?}", input).red().italic().underline()
            // );
        }
    }
}
*/
