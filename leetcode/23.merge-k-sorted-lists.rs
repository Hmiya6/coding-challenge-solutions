/*
 * @lc app=leetcode id=23 lang=rust
 *
 * [23] Merge k Sorted Lists
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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut nexts = lists;
        let mut dummy_head = Some(Box::new(ListNode::new(-1)));
        let mut tail = dummy_head.as_mut();

        loop {     
            // get an **index** of min val node
            let (i, _) = match nexts.iter()
                .map(|node| match node.as_ref() {
                    Some(v) => v.val,
                    None => std::i32::MAX,
                })
                .enumerate()
                .min_by(|x, y| {
                    let ((_, x), (_, y)) = (x, y); // compare by val
                    x.cmp(y)
                }) {
                Some(v) => v,
                None => break,
            };

            // get next node
            let mut next = nexts[i].take();
            if next.is_none() {
                break;
            }

            // get next node of next node to set new nexts
            let next_next = match next.as_mut() {
                Some(v) => v.next.take(),
                None => None, // unreachable
            };

            // set new nexts
            nexts[i] = next_next;

            // add next to return list
            tail.as_mut().unwrap().next = next;
            
            // advance tail
            tail = tail.unwrap().next.as_mut();
        }
        // return 
        dummy_head.unwrap().next
    }
}
// @lc code=end

