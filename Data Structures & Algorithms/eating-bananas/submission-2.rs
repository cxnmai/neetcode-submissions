

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut bottom = 0;
        let mut top = *piles.iter().max().unwrap();

        while bottom + 1 < top {

            let k = (bottom + top) / 2;

            let mut hours = 0;

            for &pile in piles.iter() {
                hours += (pile + k - 1) / k;
            }

            if hours <= h {
                top = k;
            } else {
                bottom = k;
            }
        }

        top
    }
}
