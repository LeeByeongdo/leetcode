struct Solution {}

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        let mut sum = 0;
        for n in nums.iter() {
            sum += n;
            res.push(sum);
        }

        res
    }
}

fn main() {
    println!("{:?}", Solution::running_sum(vec![1,2,3,4]));
}