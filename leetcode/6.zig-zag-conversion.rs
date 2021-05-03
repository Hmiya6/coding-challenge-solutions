/*
 * @lc app=leetcode id=6 lang=rust
 *
 * [6] ZigZag Conversion
 */

// @lc code=start
impl Solution {

    // 0    6      12 // <- index % 6 = 0
    // 1  5 7   11 13 // <- index % 6 = 1 or 6-1
    // 2 4  8 10      // <- index % 6 = 2 or 6-2
    // 3    9         // <- index % 6 = 3
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows.abs() as usize;
        let cycle: usize = if num_rows * 2 - 2 > 0 {
            (num_rows * 2 - 2) as usize
        } else {
            return s;
        };

        let mut result: Vec<char> = Vec::new();
        
        let mut data: Vec<Vec<char>> = vec![Vec::new(); num_rows];
        for (i, c) in s.chars().enumerate() {
            let index = if i % cycle < num_rows {
                i % cycle
            } else {
                cycle - (i % cycle)
            };
            data[index].push(c);
        }

        for mut chars in data {
            result.append(&mut chars);
        }

        result.into_iter().collect()
    }
}
// @lc code=end

