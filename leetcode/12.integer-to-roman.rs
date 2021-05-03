/*
 * @lc app=leetcode id=12 lang=rust
 *
 * [12] Integer to Roman
 */

// @lc code=start
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let mut digits = Vec::new();
        while num > 0 {
            let digit = num % 10;
            num /= 10;
            digits.push(digit);
        }

        let mut reversed = Vec::new();
        let romans = [('I', 'V'), ('X', 'L'), ('C', 'D'), ('M', '?')];
        for (log10, &digit) in digits.iter().enumerate() {
            match digit {
                0..=3 => {
                    for _ in 0..digit {
                        reversed.push(romans[log10].0);
                    }
                }
                4 => {
                    reversed.push(romans[log10].1);
                    reversed.push(romans[log10].0);
                }
                5..=8 => {
                    for _ in 5..digit {
                        reversed.push(romans[log10].0);
                    }
                    reversed.push(romans[log10].1);
                }
                9 => {
                    reversed.push(romans[log10+1].0);
                    reversed.push(romans[log10].0);
                }
                _ => {
                    panic!("Error: Invalide number: {}", digit);
                }
            }
        }
        reversed.iter().rev().collect::<String>()
    }
}
// @lc code=end

