impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut s = 0;
        let mut e = nums.len();

        loop {
            let i =(s + e) / 2;
            let c = nums[i];
            if c == target {
                return i as i32;
            } else if (e - s) <= 1 {
                break;
            } else if c > target {
                e = i;
            } else {
                s = i;
            }
        }
        -1
    }
}
