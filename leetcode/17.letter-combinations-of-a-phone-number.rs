/*
 * @lc app=leetcode id=17 lang=rust
 *
 * [17] Letter Combinations of a Phone Number
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {

    pub fn get_char(c: char) -> String {
        let res = match c {
            '2' => "abc",
            '3' => "def",
            '4' => "ghi",
            '5' => "jkl",
            '6' => "mno",
            '7' => "pqrs",
            '8' => "tuv",
            '9' => "wxyz",
            _ => panic!("WTF"),
        };
        res.into()
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        digits.chars().fold( // fold(init_val, |acc, item| -> acc) -> acc
            vec![String::new()], // <- init_val
            |acc, digit| acc.iter().flat_map(
                |acc_item| Self::get_char(digit).chars().map( // `x` is an item of acc
                    |digit_chars_item| format!("{}{}", acc_item, digit_chars_item) // `y` is an item of digit.chars()
                ).collect::<Vec<String>>() // end of `map`
            ).collect() // end of `flat_map`
        ) // end of fold
    }
}
// @lc code=end

