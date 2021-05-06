/*
 * @lc app=leetcode id=16 lang=rust
 *
 * [16] 3Sum Closest
 */

// @lc code=start
use std::cmp::Ordering;
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 { 

        let mut nums = nums;
        nums.sort();

        let mut closest_diff = std::i32::MAX;
        let mut ret = 0;

        for i in 0..(nums.len()-2) {
            let (mut left, mut right) = (i+1, nums.len()-1);
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                let sub = sum - target;
                let diff = sub.abs();
                if diff < closest_diff {
                    ret = sum;
                    closest_diff = diff;
                }
                match sub.cmp(&0) {
                    Ordering::Equal => {
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
        ret
    }
}
// @lc code=end

