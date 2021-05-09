/*
 * @lc app=leetcode id=19 lang=rust
 *
 * [19] Remove Nth Node From End of List
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode {
            val: -1,
            next: head,
        }));
        let mut len = 0;

        { // divide scope
            let mut p = dummy_head.as_ref();
            while p.unwrap().next.is_some() {
                len += 1;
                p = p.unwrap().next.as_ref();
            }
        }

        let idx = len - n;

        { // divide scope
            let mut p = dummy_head.as_mut();
            for _ in 0..idx {
                p = p.unwrap().next.as_mut();
            }
            let next = p.as_mut().unwrap() // <- parent node
                .next.as_mut().unwrap() // <- target node
                .next.take(); // <- child node
            p.as_mut().unwrap().next = next;
        }
        dummy_head.unwrap().next // return head
    }
}
// @lc code=end

