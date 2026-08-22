impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut s = 0;
        let mut e = nums.len();

        while s + 1 < e {
            let i = (s + e) / 2;
            let check = nums[i];

            if check < nums[s] && i != 0 {
                if check < nums[i - 1] {
                    return check;
                } else {
                    e = i;
                    continue;
                } 
            } else if check < nums[s] && i == 0 {
                if check < nums[nums.len() - 1] {
                    return check;
                } else {
                    e = i;
                    continue;
                }
            }
            else if check > nums[s] {
                s = i;
            }
        }

        nums[0]
    }
}
