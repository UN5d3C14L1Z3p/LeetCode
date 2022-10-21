// Valid Parentheses
//
// Easy
//
// Given a string s containing just the characters '(', ')', '{', '}', '[', determine if the input
// string is valid.
//
// An input string is valid if:
//
// Open brackets must be closed by the same type of brackets.
//
// Open brackets must be closed in the correct order.
//
// Every close bracket has a corresponding open bracket of the same type.
//
// Example 1:
//   Input: s = "()"
//   Output: true
//
// Example 2:
//
//   Input: s = "()[]{}"
//   Output: true
//
// Example 3:
//   Input: s = "(]"
//   Output: false
//
// Constraints:
//
// * 1 <= s.length <= 10^4
// * s consists of parentheses only '()[]{}'.
//
use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut t = String::from("");
        let m = HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);

        for k in s.chars() {
            if k != ')' && k != ']' && k != '}' {
                t.push(k);
            } else if !t.is_empty() {
                if let Some(c) = t.pop() {
                    if let Some(&v) = m.get(&c) {
                        if k != v {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("(]".to_string()), false);
    }
}
