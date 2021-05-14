/*
 * @lc app=leetcode id=25 lang=rust
 *
 * [25] Reverse Nodes in k-Group
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut tail = &mut head;
        for _ in 0..k {
            match tail.as_mut() {
                None => return head,
                Some(tail_ref) => tail = &mut tail_ref.next,
            }
        }

        let mut reversed_tail = Self::reverse_k_group(tail.take(), k);
        let reversed = Self::reverse_and_connect(head, reversed_tail);
        reversed
    }

    fn reverse_and_connect(head: Option<Box<ListNode>>, tail: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = tail;
        let mut current_node = head;

        while let Some(mut current_node_inner) = current_node.take() {
            let next = current_node_inner.next.take();
            current_node_inner.next = prev.take();
            prev = Some(current_node_inner);
            current_node = next;
        }
        prev.take()
    }
}
// @lc code=end

