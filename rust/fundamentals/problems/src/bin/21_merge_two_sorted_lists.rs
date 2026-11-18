use colored::*;

type Node = Option<Box<ListNode>>;

/**
 * definicion del nodo de una lista enlazada
 */
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Node,
}

/**
 * objeto solucion
 */
struct Solution;

/**
 * implementacion de la solucion
 *
 */
impl Solution {
    pub fn build(values: Vec<i32>) -> Node {
        let mut head: Node = None;
        let mut iter: &mut Node = &mut head;

        for &val in values.iter() {
            *iter = Some(Box::new(ListNode { val, next: None }));
            iter = &mut iter.as_mut().unwrap().next;
        }
        head
    }

    pub fn get_vec(head: Node) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        let mut iter = &head;
        while let Some(node) = iter {
            ans.push(node.val);
            iter = &node.next;
        }
        ans
    }

    pub fn merge_two_lists(list1: Node, list2: Node) -> Node {
        let mut l1 = Solution::get_vec(list1);
        let mut l2 = Solution::get_vec(list2);

        l1.append(&mut l2);
        l1.sort_unstable();

        Solution::build(l1)
    }
}

fn main() {
    let list1: Node = Solution::build(vec![1, 2, 4]);
    let list2: Node = Solution::build(vec![1, 3, 4]);
    let head: Node = Solution::merge_two_lists(list1, list2);
    let ans = Solution::get_vec(head);
    println!("{}", format!("{:?}", ans).green().italic().underline());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_sorted_lists() {
        let cases = [
            (
                (
                    Solution::build(vec![1, 2, 4]),
                    Solution::build(vec![1, 3, 4]),
                ),
                vec![1, 1, 2, 3, 4, 4],
            ),
            ((Solution::build(vec![]), Solution::build(vec![])), vec![]),
            ((Solution::build(vec![]), Solution::build(vec![0])), vec![0]),
        ];

        for (input, expected) in cases {
            let head = Solution::merge_two_lists(input.0.clone(), input.1.clone());
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
