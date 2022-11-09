/// Length of Last Word
///
/// Easy
///
/// Given a string s consisting of words and spaces, return the length of the last word in the
/// string.
///
/// A word is a maximal substring consisting of non-space characters only.
///
/// Example !:
///   Input: s = "Hello World"
///   Output: 5
///   Explanation: The last word is "World" with length 5.
///
/// Example 2:
///   Input: s = "   fly me   to    the moon "
///   Output: 4
///   Explanation: The last word is "moon" with length 4.
///
/// Example 3:
///   Input: s = "luffy is still joyboy"
///   Output: 6
///   Explanation: The last word is "joyboy" with length 6.
///
/// Constraints:
///
/// * 1 <= s.length <= 10^4
/// * s consists of only English letters and spaces ' '.
/// * There will be at least one word in s.
///
struct Solution {}
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        match s.trim().split(' ').collect::<Vec<_>>().last() {
            Some(s) => s.len() as i32,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_last_word() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(
            Solution::length_of_last_word("    fly me  to  the moon ".to_string()),
            4
        );
        assert_eq!(
            Solution::length_of_last_word("luffy is still joyboy".to_string()),
            6
        );
    }
}
