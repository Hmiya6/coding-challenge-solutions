/*
 * @lc app=leetcode id=30 lang=rust
 *
 * [30] Substring with Concatenation of All Words
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if words.len() == 0 || s.len() < words.len() * words[0].len() {
            return vec![];
        }

        let len = words.len() * words[0].len();
        let map = words
            .iter()
            .fold(HashMap::<String, i32>::new(), |mut hmap, word| {
                *hmap.entry(word).or_insert(0) += 1;
                hmap                
            });

        (0..=(s.len() - len))
            .filter(|index| is_permutation())
    }

    fn is_permutation(s: &str, map: &HashMap<String, i32>, words: &[String]) -> bool {
        let mut new_map = HashMap::new();
        let len = words[0].len();

        for i in (0..s.len()).step_by(len) {
            let word = s[i..(i+len)];
            *new_map.entry(word).or_insert(0) += 1;

            if new_map.get(word) > map.get(word) {
                return false;
            }
        }

        true
    }
}
// @lc code=end

