/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
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


// 2 non-empty linked lists non-negative integers
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        
        let mut overflow = 0;
        let mut res_stack = Vec::new();
        loop {
            if l1.is_none() && l2.is_none() && overflow == 0 {
                break;
            }
            let mut digit1 = 0;
            match l1 {
                Some(node) => {
                    digit1 = node.val;
                    l1 = node.next;
                },
                None => {
                }
            }
            let mut digit2 = 0;
            match l2 {
                Some(node) => {
                    digit2 = node.val;
                    l2 = node.next;
                }
                None => {
                }
            }
            let sum = digit1 + digit2 + overflow;
            overflow = sum/10;
            let digit = sum  % 10;
            res_stack.push(digit);
        }
        res_stack.reverse();
        
        let mut new_node = ListNode::new(0);
        let mut old_node = None;
        for digit in res_stack {
            new_node = ListNode::new(digit);
            new_node.next = old_node;
            old_node = Some(Box::new(new_node));
        }
        old_node
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tset() {
        
    }
}