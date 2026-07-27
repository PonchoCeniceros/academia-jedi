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
 *
 * Una lista enlazada en memoria se ve así:
 *
 *   head
 *    │
 *    ▼
 *   Some┌─────┬──────┐   Some┌─────┬──────┐   None
 *       │ val │ next │──────▶│ val │ next │──────▶
 *       │  1  │  ●   │       │  2  │  ●   │
 *       └─────┴──────┘       └─────┴──────┘
 *
 * `Node` es un Option: o hay un nodo (Some) o se acabó la lista (None).
 * `Box<ListNode>` es el nodo "real" viviendo en el heap.
 */
impl Solution {
    pub fn build(values: Vec<i32>) -> Node {
        // Inicializas la cabeza de la lista enlazada (todavía vacía = None)
        let mut head: Node = None;

        // `iter` aquí es &mut Node: una referencia MUTABLE, el "puntero láser"
        // que apunta al hueco donde hay que escribir el próximo nodo.
        //
        //   head: None
        //          ▲
        //   iter ──┘   (iter = &mut head)
        let mut iter: &mut Node = &mut head;

        for &val in values.iter() {
            // *iter = ...  ->  no cambiamos la referencia, cambiamos
            // el CONTENIDO al que apunta. Es como decir:
            // "en el hueco que estoy señalando, pon este nodo nuevo".
            //
            //   antes:  iter ──▶ [ None ]
            //   *iter = Some(Box{val, next: None})
            //   después: iter ──▶ [ Some(Box{val, next: None}) ]
            *iter = Some(Box::new(ListNode { val, next: None }));

            // Ahora hay que mover el "láser" para que apunte al
            // siguiente hueco (el campo `next` del nodo que acabamos
            // de crear), sin robarle la propiedad al nodo:
            //
            //   iter.as_mut()  : &mut Option<Box<ListNode>>
            //                       -> Option<&mut Box<ListNode>>
            //   .unwrap()      : &mut Box<ListNode>   (ya sabemos que es Some)
            //   .next          : campo next dentro de ese Box
            //   &mut ...next   : &mut Node  <- nuevo destino de iter
            //
            //   head:  Some( val:1, next: None )
            //                        ▲
            //           iter ────────┘  (apunta al hueco "next" de nodo 1)
            iter = &mut iter.as_mut().unwrap().next;
        }
        head
    }

    pub fn get_vec(head: Node) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        // `iter` es una REFERENCIA (&) al Node actual, no el dueño.
        // Piensa en `iter` como un dedo que va señalando nodos,
        // sin "tomar prestado para siempre" ni mover nada.
        //
        //   head: Some(1)->Some(2)->None
        //          ▲
        //   iter ──┘   (iter = &head)
        let mut iter = &head;

        // `while let Some(node) = iter` intenta "abrir" el Option:
        // - si iter apunta a Some(nodo), `node` queda ligado a ese Box<ListNode>
        // - si iter apunta a None, el while termina
        //
        //   iter: &Some(Box{val:1, next: Some(Box{val:2,...})})
        //                     │
        //                     ▼
        //   node: &Box{val:1, next: Some(Box{val:2,...})}   <- referencia al nodo
        while let Some(node) = iter {
            ans.push(node.val);

            // avanzamos el dedo: ahora iter apunta al campo `next`
            // del nodo actual, es decir, al SIGUIENTE Node.
            //
            //   antes:  iter ──▶ Node(1) ──▶ Node(2) ──▶ None
            //                     ▲
            //   después: iter ────┘apunta ahora a Node(2)
            //
            //   iter ──▶ Node(1) ──▶ Node(2) ──▶ None
            //                         ▲
            //                iter ────┘
            iter = &node.next;
        }
        ans
    }

    pub fn reverse_list(head: Node) -> Node {
        let mut queue: Vec<i32> = vec![];

        // extraccion de los valores de la lista enlazada
        // en una queue
        let mut iter: &Node = &head;
        while let Some(node) = iter {
            queue.insert(0, node.val);
            iter = &node.next;
        }

        // armado de una nueva lista enlazada con los
        // valores extraidos de la queue
        let mut new_head: Node = None;
        let mut new_iter: &mut Node = &mut new_head;
        for &val in queue.iter() {
            *new_iter = Some(Box::new(ListNode::new(val)));
            new_iter = &mut new_iter.as_mut().unwrap().next;
        }
        new_head
    }
}

fn main() {
    let ll0: Node = Solution::build(vec![1, 2, 3, 4, 5]);
    println!("{}", format!("{:?}", ll0).purple().italic().underline());
    let ans: Option<Box<ListNode>> = Solution::reverse_list(ll0);
    println!("{}", format!("{:?}", ans).green().italic().underline());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_linked_list() {
        let cases = [
            (Solution::build(vec![1, 2, 3, 4, 5]), vec![5, 4, 3, 2, 1]),
            (Solution::build(vec![1, 2]), vec![2, 1]),
            (Solution::build(vec![]), vec![]),
        ];

        for (input, expected) in cases {
            let head = Solution::reverse_list(input.clone());
            let ans = Solution::get_vec(head);

            assert_eq!(
                ans,
                expected,
                "{}",
                format!("{:?}", input).red().italic().underline()
            );
        }
    }
}
