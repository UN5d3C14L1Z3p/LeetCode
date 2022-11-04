/// Search Insert Position
///
/// Easy
///
/// Given a sorted array of distinct integers and a target value, return the index if the target is
/// found. If not, return the index where it would be if it were inserted in order.
///
/// You must write an algorithm with O(log n) runtime complexity.
///
/// Example 1:
///   Input: nums = [1,3,5,6], target = 5
///   Output: 2
///
/// Example 2:
///   Input: nums = [1,3,5,6], target = 2
///   Output: 1
///
/// Example 3:
///   Input: nums = [1,3,5,6], target = 7
///   Output: 7
///
/// Constraints:
/// * 1 <= nums.length <= 10^4
/// * -10^4 <= nums[i] <= 10^4
/// * nums contains distinct values sorted in ascending order.
/// * -10^4 <= target <= 10^4
///
use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let middle = nums.len() / 2;
        if let Some(&e) = nums.get(middle) {
            match target.cmp(&e) {
                Ordering::Greater => {
                    // return Self::search_insert(nums.as_slice()[..middle].to_vec(), target);
                    1
                }
                Ordering::Less => {
                    // return Self::search_insert(nums.as_slice()[middle..].to_vec(), target);
                    2
                }
                Ordering::Equal => middle.try_into().unwrap(),
            }
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 7);
    }
}
