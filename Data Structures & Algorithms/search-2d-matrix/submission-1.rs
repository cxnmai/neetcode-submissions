impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut s_row = 0;
        let mut e_row = matrix.len();

        'outer: loop {
            let i_row = (s_row + e_row) / 2;
            let row = &matrix[i_row];

            let mut s: usize = 0;
            let mut e = row.len();

            if row[0] > target {
                e_row = i_row;
                if i_row != 0 {
                    if target > *matrix[i_row - 1].last().unwrap() {
                        break;
                    }
                } else {
                    break;
                }
            } else if row[row.len() - 1] < target {
                s_row = i_row;
                if i_row != matrix.len() - 1 {
                    if target < matrix[i_row + 1][0] {
                        break;
                    }
                } else {
                    break;
                }
            } else { loop {
                let i = (s + e) / 2;
                let c = row[i];
                if c == target {
                    return true;
                } else if (e - s) <= 1 {
                    break 'outer;
                } else if c > target {
                    e = i;
                } else {
                    s = i;
                }
            } }
        }

        false
    }
}
