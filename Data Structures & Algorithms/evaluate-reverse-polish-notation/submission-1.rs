impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();

        for token in tokens.iter() {
            
            if let Ok(num) = token.parse() {
                stack.push(num);
            } else {

                let last = stack.pop().unwrap();
                let second_last = stack.pop().unwrap();


                match token.as_str() {
                    "+" => stack.push(second_last + last),
                    "-" => stack.push(second_last - last),
                    "*" => stack.push(second_last * last),
                    "/" => stack.push(second_last / last),
                    _ => {}
                }

            }
        }
        *stack.last().unwrap()
    }
}
