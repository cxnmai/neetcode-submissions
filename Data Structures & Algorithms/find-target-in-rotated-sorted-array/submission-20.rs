impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut s = 0;
        let mut e = nums.len();

        while s + 1 < e {
            let i = (s + e) / 2;
            let check = nums[i];
            println!(
                "start: {}\t check: {}\t end (incl): {}",
                nums[s], check, nums[e - 1]
            );

            if check == target {
                return i as i32;
            }

            if nums[s] <= check {
            // Left half is sorted
            if nums[s] <= target && target < check {
                e = i;
            } else {
                s = i;
            }
        } else {
            // Right half is sorted
            if check < target && target <= nums[e - 1] {
                s = i;
            } else {
                e = i;
            }
        }
        }
        if nums[s] == target {
            return s as i32;
        } else if nums[e - 1] == target {
            return (e - 1) as i32;
        }

        -1
    }
}
