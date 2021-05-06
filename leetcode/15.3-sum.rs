/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

// @lc code=start
use std::cmp::Ordering;
use std::collections::HashSet;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        } 
        
        let mut nums = nums;
        nums.sort();

        let mut set = HashSet::new();

        for i in 0..(nums.len()-2) {
            let (mut left, mut right) = (i+1, nums.len()-1);
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                match sum.cmp(&0) {
                    Ordering::Equal => {
                        set.insert(vec![nums[i], nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                    }
                    Ordering::Greater => {
                        right -= 1;
                    }
                    Ordering::Less => {
                        left += 1;
                    }
                }
            }
        }
        set.iter().cloned().collect()
    }
}
// @lc code=end

