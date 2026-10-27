/*
2215. Find the Difference of Two Arrays
Difficulty: Easy
https://leetcode.com/problems/find-the-difference-of-two-arrays/

Given two 0-indexed integer arrays nums1 and nums2, return a list
answer of size 2 where:

- answer[0] is a list of all distinct integers in nums1 which are
  not present in nums2.
- answer[1] is a list of all distinct integers in nums2 which are
  not present in nums1.

Note: The integers in the lists may be returned in any order.

Example 1:

Input: nums1 = [1,2,3], nums2 = [2,4,6]
Output: [[1,3],[4,6]]

Explanation:
For nums1, nums1[1] = 2 is present at index 0 of nums2, whereas
nums1[0] = 1 and nums1[2] = 3 are not present in nums2.
Therefore, answer[0] = [1,3].

For nums2, nums2[0] = 2 is present at index 1 of nums1, whereas
nums2[1] = 4 and nums2[2] = 6 are not present in nums1.
Therefore, answer[1] = [4,6].

Example 2:

Input: nums1 = [1,2,3,3], nums2 = [1,1,2,2]
Output: [[3],[]]

Explanation:
For nums1, nums1[2] and nums1[3] are not present in nums2.
Since nums1[2] == nums1[3], their value is only included once,
so answer[0] = [3].

Every integer in nums2 is present in nums1.
Therefore, answer[1] = [].

Constraints:

- 1 <= nums1.length, nums2.length <= 1000
- -1000 <= nums1[i], nums2[i] <= 1000
*/

use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let set_a: HashSet<i32> = nums1.iter().copied().collect();
        let set_b: HashSet<i32> = nums2.iter().copied().collect();

        let mut res_a: Vec<i32> = Vec::new();
        let mut res_b: Vec<i32> = Vec::new();

        
        let set_a_iter: Vec<i32> = set_a.iter().copied().collect();
        for i in 0..(set_a_iter.len()) {
            let val: &i32 = set_a_iter.get(i).unwrap();
            if !set_b.contains(val) {
                res_a.push(*val);
            }
        }

        let set_b_iter: Vec<i32> = set_b.iter().copied().collect();
        for i in 0..(set_b_iter.len()) {
            let val: &i32 = set_b_iter.get(i).unwrap();
            if !set_a.contains(val) {
                res_b.push(*val);
            }
        }

        return vec![res_a, res_b];
    }
}

fn main() {
    // Example 1
    let nums1 = vec![1, 2, 3];
    let nums2 = vec![2, 4, 6];

    let result = Solution::find_difference(nums1, nums2);

    println!("Result: {:?}", result);
    println!("Expected: [[1, 3], [4, 6]]");

    // Example 2
    let nums1 = vec![1, 2, 3, 3];
    let nums2 = vec![1, 1, 2, 2];

    let result = Solution::find_difference(nums1, nums2);

    println!("Result: {:?}", result);
    println!("Expected: [[3], []]");
}
