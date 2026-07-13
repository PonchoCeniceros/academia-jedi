use colored::*;

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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        println!("{:?}", head);
        None
    }
}

fn main() {
    let ll0: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 1, next: None }));
    let ans: Option<Box<ListNode>> = Solution::reverse_list(ll0);
    println!("{}", format!("{:?}", ans).green().italic().underline());
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
