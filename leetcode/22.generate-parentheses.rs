/*
 * @lc app=leetcode id=22 lang=rust
 *
 * [22] Generate Parentheses
 */

// @lc code=start
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        
        fn dfs(ret: &mut Vec<String>, open: i32, close:i32, sublist: String) {
            if open == 0 && close == 0 {
                return ret.push(sublist);
            }
            if open > 0 {
                dfs(ret, open-1, close, format!("{}{}", sublist, "("));
            }
            if close > open {
                dfs(ret, open, close-1, format!("{}{}", sublist, ")"));
            }
        }

        let mut ret = vec![];
        dfs(&mut ret, n, n, "".to_string());
        ret 
    }
}
// @lc code=end

