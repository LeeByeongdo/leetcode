struct Solution {}

impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut clone = arr.to_vec();
        clone.sort();

        let mut gap = 0;
        let mut prev = 0;

        let mut res = true;
        for (idx, v) in clone.into_iter().enumerate() {
            if idx == 0 {
                prev = v;
                continue;
            }

            let diff = v - prev;
            if idx == 1 {
                gap = diff;
                prev = v;
                continue;
            }

            if gap != diff {
                res = false;
                break;
            }

            prev = v;
            gap = diff;
        };

        res
    }
}

fn main() {
    let res = Solution::can_make_arithmetic_progression(vec![1,2,4]);
    println!("{}", res);
}