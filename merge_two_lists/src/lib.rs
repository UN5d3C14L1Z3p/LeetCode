// Merge Two Sorted Lists
//
// Easy
//
// You are given the heads of two sorted linked lists list1 and list2.
//
// Merge the two lists in a one sorted list. The list should be made by splicing together the nodes
// of the first two lists.
//
// Return the head of the merged linked list.
//
// Example 1:
//   (1) -> (2) -> (4)
//   (1) -> (3) -> (4)
// ---------------------
// (1) -> (1) -> (2) -> (3) -> (4) -> (4)
//
//  Input: list1 = [1,2,4], list2 = [1,3,4]
//  Output: [1,1,2,3,4,4]
//
// Example 2:
//   Input: list1 = [], list2 = []
//   Output: []
//
// Example 3:
//   Input: list1 = [], list2 = [0]
//   Output: [0]
//
// Constraints:
//
// * The number of nodes in both lists is in the range [0, 50].
// * -100 <= Node.val <= 100
// * Both list1 and list2 are sorted in non-decreasing order.
//

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(list1), Some(list2)) => {
                if list1.val >= list2.val {
                    Some(Box::new(ListNode {
                        val: list2.val,
                        next: Solution::merge_two_lists(Some(list1), list2.next),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: list1.val,
                        next: Solution::merge_two_lists(list1.next, Some(list2)),
                    }))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_lists() {
        let list1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let list2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let list3 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 4, next: None })),
                        })),
                    })),
                })),
            })),
        }));

        assert_eq!(Solution::merge_two_lists(list1, list2), list3);

        let list1 = Some(Box::new(ListNode::default()));
        let list2 = Some(Box::new(ListNode::default()));
        let list3 = Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode { val: 0, next: None })),
        }));
        assert_eq!(Solution::merge_two_lists(list1, list2), list3);
    }
}
