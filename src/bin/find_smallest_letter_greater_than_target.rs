struct Solution {}

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        return if let Some(&v) = letters.iter().find(|&&l| l > target) {
            v
        } else {
            letters[0]
        }
    }
}

fn main() {
    println!("{}", Solution::next_greatest_letter(vec!['c','f','j'], 'c'));
}