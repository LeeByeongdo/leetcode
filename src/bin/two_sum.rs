use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for (pos, n) in nums.iter().enumerate() {
            let sub = target - n;
            if map.get(&sub).is_none() {
                map.insert(*n, pos);
            } else {
                return vec![pos as i32, *map.get(&sub).unwrap() as i32];
            }
        }

        panic!("no answer!")
    }
}

fn main() {
    let res = Solution::two_sum(vec![2,7,11,15], 9);
    println!("{:?}", res);
    println!("{:?}", Solution::two_sum(vec![3,2,4], 6));
    println!("{:?}", Solution::two_sum(vec![3,3], 6));
}