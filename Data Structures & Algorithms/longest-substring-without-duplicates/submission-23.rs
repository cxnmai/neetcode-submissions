impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes: &[u8] = s.as_bytes();

        let mut in_window = [false; 128];
        let mut l: usize = 0;
        let mut best: usize = 0;

        for (r, &c) in bytes.iter().enumerate() {
            while in_window[c as usize] {
                in_window[bytes[l] as usize] = false;
                l += 1;
            }
            in_window[c as usize] = true;
            best = best.max(r - l + 1);
        }
        best as i32
    }
}