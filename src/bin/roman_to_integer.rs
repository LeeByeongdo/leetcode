struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let r: Vec<char> = s.chars().rev().collect();
        let mut prev = c_to_num(r[0]);
        let mut result = prev;

        r.iter().skip(1).for_each(|&c| {
            let v = c_to_num(c);
            if v < prev {
                result -= v;
            } else {
                result += v;
            }
            prev = v;
        });

        result
    }
}

fn c_to_num(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => {
            panic!("invalid char!")
        }
    }
}

fn main() {
    println!("{}", Solution::roman_to_int(String::from("III")));
    println!("{}", Solution::roman_to_int(String::from("LVIII")));
    println!("{}", Solution::roman_to_int(String::from("MCMXCIV")));
}