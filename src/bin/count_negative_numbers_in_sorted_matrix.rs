struct Solution {}

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();
        let mut res = 0;
        for v in grid.iter() {
            for (pos, &num) in v.iter().enumerate() {
                if num < 0 {
                    res += n - pos;
                    break;
                }
            }
        }

        res as i32
    }
}

fn main() {
    println!("{}", Solution::count_negatives(vec![vec![4,3,2,-1], vec![3,2,1,-1], vec![1,1,-1,-2], vec![-1,-1,-2,-3]]));
}