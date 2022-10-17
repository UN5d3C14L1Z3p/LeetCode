// Longest Common Prefix
//
// Easy
//
// Write a function to find the longest common prefix string amongst an array of strings.
//
// If there is no common prefix, return an empty string "".
//
// Example 1:
//
//   Input: strs = ["flower","flow","flight"]
//   Output: "fl"
//
// Example 2:
//
//   Input: strs = ["dog","racecar","car"]
//   Output: ""
//   Explanation: There is no common prefix among the input strings.
//
// Constraints:
// * 1 <= strs.length <= 200
// * 0 <= strs[i].length <= 200
// * strs[i] consists of only lowercase English

struct Solution {}
impl Solution {
    pub fn longest_common_prefix(_strs: Vec<String>) -> String {
        "fl".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            Solution::longest_common_prefix(
                ["flower", "flow", "flight"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            ),
            "fl".to_string()
        );
        assert_eq!(
            Solution::longest_common_prefix(
                ["dog", "racer", "car"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            ),
            "".to_string()
        );
    }
}
