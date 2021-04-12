/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

// @lc code=starat
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest = 0;
        let svec = s.chars().collect::<Vec<char>>();
        for i in 0..svec.len() {
            // -------------------------------------
            let mut char_dic = HashMap::new();
            for j in i..svec.len() {
                if char_dic.contains_key(&svec[j]) {
                    break;
                } else {
                    // println!("{}, {}: {}", i, j, svec[j]);
                    char_dic.insert(svec[j], ());
                    let diff = j - i + 1;
                    if diff > longest {
                        longest = diff;
                    }
                }
            }
            // ------------------------------------
        }
        longest as i32
    }
}
// @lc code=end

