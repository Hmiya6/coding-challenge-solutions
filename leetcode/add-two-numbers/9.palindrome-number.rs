/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let original = x;        
        let mut x = x;
        let mut reversed = 0;
        while x > 0 {
            let digit = x % 10;
            x /= 10;
            reversed = reversed*10 + digit;
        }
        return (original == reversed)
    }
}
// @lc code=end

