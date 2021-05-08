/*
 * @lc app=leetcode id=18 lang=rust
 *
 * [18] 4Sum
 */

// @lc code=start
use std::cmp::Ordering;
use std::collections::HashSet;
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }

        let mut nums = nums;
        nums.sort();
        
        let mut ret = vec![];        
        for i in 0..nums.len() -3 {

            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }

            for j in i+1..nums.len() -2 {

                if j > i+1 && nums[j] == nums[j-1] {
                    continue;
                }

                let sub_target = target - (nums[i] + nums[j]);

                let (mut left, mut right) = (j+1, nums.len()-1);

                while left < right {

                    if left > j+1 && nums[left] == nums[left-1] {
                        left += 1;
                        continue;
                    }
                    if right < nums.len() -1 && nums[right] == nums[right+1] {
                        right -= 1;
                        continue;
                    }

                    let sub_sum = nums[left] + nums[right];

                    match sub_sum.cmp(&sub_target) {
                        Ordering::Equal => {
                            ret.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                            left += 1;
                            right -= 1;
                        }
                        Ordering::Less => {
                            left += 1;
                        }
                        Ordering::Greater => {
                            right -= 1;
                        }
                    }
                }
            }
        }
        ret
    }
}
// @lc code=end

