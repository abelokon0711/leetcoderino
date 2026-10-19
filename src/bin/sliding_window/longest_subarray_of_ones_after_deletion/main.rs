/*
 * 1493. Longest Subarray of 1's After Deleting One Element
 * https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element/
 *
 * Difficulty: Medium
 *
 * You are given a binary array `nums`, you should delete one element from it.
 *
 * Return the size of the longest non-empty subarray containing only 1's
 * in the resulting array. Return 0 if there is no such subarray.
 *
 * Example 1:
 * Input: nums = [1,1,0,1]
 * Output: 3
 * Explanation: After deleting the number at position 2, [1,1,1]
 * contains 3 numbers with a value of 1.
 *
 * Example 2:
 * Input: nums = [0,1,1,1,0,1,1,0,1]
 * Output: 5
 * Explanation: After deleting the number at position 4,
 * [0,1,1,1,1,1,0,1] has a longest subarray of [1,1,1,1,1].
 *
 * Example 3:
 * Input: nums = [1,1,1]
 * Output: 2
 * Explanation: You must delete one element.
 *
 * Constraints:
 * - 1 <= nums.length <= 10^5
 * - nums[i] is either 0 or 1.
 */

pub struct Solution {}

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let k: i32 = 1;
        let mut l: usize = 0;
        let mut r: usize = 0;
        let mut used_k: i32 = 0;
        let mut curr: i32 = 0;
        let mut max: i32 = 0;

        while r < nums.len() {
            if nums[r] == 0 {
                used_k += 1;
            }
            curr +=1;
            r += 1;

            if used_k > k {
                if nums[l] == 0 {
                    used_k -= 1;
                }
                curr -= 1;
                l += 1;
            }

            if curr > max {
                max = curr;
            }
        }

        return max - 1;
    }
}

fn main() { 
    let nums = vec![0,1,1,1,0,1,1,0,1];

    let result = Solution::longest_subarray(nums);

    println!("Result: {}", result);
    println!("Expected: 5");
}