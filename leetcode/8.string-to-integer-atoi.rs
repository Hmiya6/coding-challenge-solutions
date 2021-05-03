/*
 * @lc app=leetcode id=8 lang=rust
 *
 * [8] String to Integer (atoi)
 */

// @lc code=start

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut chars = s.chars().peekable();
        let mut result: u128 = 0;
        let mut is_negative = false;
        let mut expects_num = false;
        loop {
            match chars.peek() {
                Some(&c) => {
                    match c {
                        ' ' | '\t' => {
                            chars.next();
                            if expects_num {
                                break;
                            }
                        },
                        '0'..='9' => {
                            chars.next();
                            let num = c.to_digit(10).unwrap() as u128;
                            result = result*10 + num;
                            expects_num = true;
                        }
                        '+' | '-' => {
                            chars.next();
                            if expects_num {
                                break;
                            }
                            expects_num = true;
                            if c == '-' {
                                is_negative = true;
                            }
                        }
                        _ => {
                            break;
                        }

                    }
                }
                None => break,
            }
        }
        
        match (is_negative, result > u128::pow(2, 31)) {
            (true, true) => {
                -1*u128::pow(2, 31) as i32
            }
            (true, false) => {
                -1*result as i32
            }
            (false, true) => {
                (u128::pow(2, 31) as i64 - 1) as i32
            }
            (false, false) => {
                if result > u128::pow(2, 31) -1 {
                    return (u128::pow(2, 31) -1) as i32;
                }
                result as i32
            }
        }
    }
}
// @lc code=end

