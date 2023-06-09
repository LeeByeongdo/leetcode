struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for c in s.chars() {
            match c {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                ')' | '}' | ']' => {
                  if let Some(p) = stack.pop() {
                      if c != p {
                          return false;
                      }
                  } else {
                      return false;
                  }
                },
                _ => {}
            }

        }

        stack.is_empty()
    }
}

fn main() {
    println!("{}", Solution::is_valid(String::from("()")));
    println!("{}", Solution::is_valid(String::from("()[]{}")));
    println!("{}", Solution::is_valid(String::from("(]")));
}