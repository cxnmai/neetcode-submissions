impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {

        let mut stack = vec![vec![temperatures[0], 0]];

        let mut res = vec![0; temperatures.len()];

        for i in 1..temperatures.len() {
            let cur = temperatures[i];

            while !stack.is_empty() {
                let last = stack.last().unwrap()[0];

                if cur > last {
                    res[stack.last().unwrap()[1] as usize] = i as i32 - stack.last().unwrap()[1];
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push(vec![cur, i as i32])
        }
        
        res
    }
}
