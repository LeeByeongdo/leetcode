struct Solution {}

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let size = grid.len();

        for i in 0..size {
            let mut v = vec![];
            for j in 0..size {
                v.push(grid[j][i]);
            }

            for row in grid.iter() {
                if row.eq(&v) {
                    count += 1;
                }
            }
        }

        count
    }
}

fn main() {
    println!("{}", 100_001);
    println!("{}", Solution::equal_pairs(vec![vec![3,2,1], vec![1,7,6], vec![2,7,7]]));
}