struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut prev = prices[0];
        let mut max = 0;

        for i in prices[1..].iter() {
            if *i < prev {
                prev = *i;
            } else {
                max = max.max(*i - prev);
            }
        }

        max
    }
}

fn main() {
    println!("{}", Solution::max_profit(vec![7,1,5,3,6,4]));
    println!("{}", Solution::max_profit(vec![7,6,4,3,1]));

}