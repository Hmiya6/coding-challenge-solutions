/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        
        let mut s = s;
        let chs = s.chars().collect::<Vec<char>>();
        let mut dp = vec![vec![false; chs.len()]; chs.len()];
        
        let mut longest_len = 1;
        let mut start = 0;
        let mut end = 0;

        for i in 0..chs.len() {
            dp[i][i] = true;
            if i != 0 {
                dp[i-1][i] = (chs[i-1] == chs[i]);
                if dp[i-1][i] && longest_len < 2 {
                    start = i-1;
                    end = i;
                    longest_len = 2;
                }
            }
            
        }
        if chs.len() < 3 {
            return s.drain(start..=end).collect::<String>();
        }
        
        for i in (0..chs.len()-2).rev() {
            for j in (i+2..chs.len()).rev() {
                dp[i][j] = (chs[i] == chs[j] && dp[i+1][j-1]);
                if dp[i][j] && longest_len < (j - i) + 1 {
                    start = i;
                    end = j;
                    longest_len = end - start + 1;
                }
            } 
        }
        s.drain(start..=end).collect::<String>()
    }
}
// @lc code=end

