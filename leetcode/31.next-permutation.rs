/*
 * @lc app=leetcode id=31 lang=rust
 *
 * [31] Next Permutation
 */

// @lc code=start
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        // 1 2 5 3
        // <- 1 2 3 5
        // -> 1 3 2 5
        if let Some(i) = (1..nums.len()).rev().find(|&i| nums[i-1] < nums[i]) {
            let j = (i..nums.len()).rev().find(|&j| nums[i-1] < nums[j]).unwrap();
            nums.swap(i-1, j);
            nums[i..].reverse();
        } else {
            nums.reverse();
        };
    }
}
// @lc code=end
