struct Solution {}

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut arr = arr.clone();
        arr.sort();

        let mut counts = vec![];

        let mut prev = arr[0];
        let mut cur_count: usize = 1;

        for &v in arr.iter().skip(1) {
           if prev == v {
               cur_count += 1;
           } else {
               let has= counts.contains(&cur_count);

               if has {
                   return false;
               } else {
                   counts.push(cur_count);
                   prev = v;
                   cur_count = 1;
               }
           }
        }

        return !counts.contains(&cur_count);
    }
}

fn main() {
    println!("{}", Solution::unique_occurrences(vec![26,2,16,16,5,5,26,2,5,20,20,5,2,20,2,2,20,2,16,20,16,17,16,2,16,20,26,16]));
}