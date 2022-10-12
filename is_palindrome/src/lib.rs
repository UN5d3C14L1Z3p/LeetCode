// Easy
//
// Palindrome Number
//
// Given an integer x, return true if x is palindrome integer.
// An integer is a palindrome when it reads the same backward  as forward.
//
// For example, 121 is a palindrome while 123 is not.
//
// Example 1:
// Input: x = 121
// Output: true
// Explanation: 121 reads as 121 from left to right and from right to left.
//
// Example 2:
// Input: x = -121
// Output: false
// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. There it is
// not a palindrome.
//
// Example 3:
// Input: x = 10
// Output: false
// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
//
// Constraints:
// * -2^31 <= x <= 2^31 - 1
//
// Follow-up: Coud you solve it without converting the integer to a string?

struct Solution {}

impl Solution {
    fn reverse(x: i32) -> i32 {
        let mut n = x;
        let mut reversed = 0;

        while n != 0 {
            reversed = reversed * 10 + n % 10;
            n /= 10;
        }

        reversed
    }

    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            false
        } else {
            x == Self::reverse(x)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(Solution::is_palindrome(121));
        assert!(!Solution::is_palindrome(-121));
        assert!(!Solution::is_palindrome(10));
    }
}
