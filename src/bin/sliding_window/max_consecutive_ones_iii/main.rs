/*
1004. Max Consecutive Ones III
https://leetcode.com/problems/max-consecutive-ones-iii/

Difficulty: Medium

Given a binary array nums and an integer k, return the maximum number
of consecutive 1's in the array if you can flip at most k 0's.

Example 1:
Input:  nums = [1,1,1,0,0,0,1,1,1,1,0], k = 2
Output: 6
Explanation:
[1,1,1,0,0,1,1,1,1,1,1]
           ^         ^
The marked numbers were flipped from 0 to 1.
The longest consecutive sequence of 1's has length 6.

Example 2:
Input:  nums = [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], k = 3
Output: 10
Explanation:
[0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,1,1,1,1]
         ^ ^       ^
The marked numbers were flipped from 0 to 1.
The longest consecutive sequence of 1's has length 10.

Constraints:
- 1 <= nums.length <= 10^5
- nums[i] is either 0 or 1.
- 0 <= k <= nums.length
*/

pub struct Solution {}

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
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

        return max;
    }
}

fn main() {
    let nums = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
    let k = 2;

    let result = Solution::longest_ones(nums, k);

    println!("Result: {}", result);
    println!("Expected: 6");
}