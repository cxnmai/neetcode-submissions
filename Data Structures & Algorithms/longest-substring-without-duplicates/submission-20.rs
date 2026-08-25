use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {

        let mut window = HashMap::new();

        let mut res = 0;

        let mut start = 0;

        for (i, c) in s.chars().enumerate() {
            if let Some(j) = window.get(&c) {
                if *j >= start {
                    start = j + 1;
                }
            }

            window.insert(c, i);

            res = std::cmp::max(res, i + 1 - start);
        }

        res as i32
    }
}
