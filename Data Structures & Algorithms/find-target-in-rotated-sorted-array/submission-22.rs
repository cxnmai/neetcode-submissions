impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut s = 0;
        let mut e = nums.len();

        while s < e {
            let i = (s + e) / 2;
            let check = nums[i];

            if check == target {
                return i as i32;
            }

            if nums[s] <= check {
                // Left half is sorted
                if nums[s] <= target && target < check {
                    e = i;
                } else {
                    s = i + 1;
                }
            } else {
                // Right half is sorted
                if check < target && target <= nums[e - 1] {
                    s = i + 1;
                } else {
                    e = i;
                }
            }
        }

        -1
    }
}
