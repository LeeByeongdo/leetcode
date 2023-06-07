struct Solution {}

impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let a_bits = format!("{:b}", a);
        let b_bits = format!("{:b}", b);
        let c_bits = format!("{:b}", c);

        let max = a_bits.len().max(b_bits.len()).max(c_bits.len());

        let a_bits = format!("{:0width$b}", a, width=max);
        let b_bits = format!("{:0width$b}", b, width=max);
        let c_bits = format!("{:0width$b}", c, width=max);

        let a_bits: Vec<char> = a_bits.chars().collect();
        let b_bits: Vec<char> = b_bits.chars().collect();
        let c_bits: Vec<char> = c_bits.chars().collect();

        let mut count = 0;
        for i in 0..max {
            if c_bits[i] == '0' {
                if a_bits[i] == '1' {
                    count += 1;
                }

                if b_bits[i] == '1' {
                    count += 1;
                }
            } else {
                if a_bits[i] == '0' && b_bits[i] == '0' {
                    count += 1;
                }
            }
        }

        count
    }
}

fn main() {
    println!("{}", Solution::min_flips(8,3,5));
}