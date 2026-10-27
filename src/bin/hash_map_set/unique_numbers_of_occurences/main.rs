/*
 * LeetCode 1207 - Unique Number of Occurrences
 * Difficulty: Easy
 *
 * Given an array of integers arr, return true if the number
 * of occurrences of each distinct value is unique.
 * Otherwise, return false.
 *
 * Example 1:
 * Input: arr = [1,2,2,1,1,3]
 * Output: true
 * Explanation:
 * 1 occurs 3 times, 2 occurs 2 times, and 3 occurs 1 time.
 * All occurrence counts are unique.
 *
 * Example 2:
 * Input: arr = [1,2]
 * Output: false
 * Explanation:
 * Both 1 and 2 occur once.
 *
 * Example 3:
 * Input: arr = [-3,0,1,-3,1,1,1,-3,10,0]
 * Output: true
 *
 * Constraints:
 * 1 <= arr.length <= 1000
 * -1000 <= arr[i] <= 1000
 */

use std::collections::HashMap;
use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map_a: HashMap<i32, i32> = HashMap::new();

        for i in 0..(arr.len()) {
            let val = arr.get(i).unwrap();
            if map_a.contains_key(val) {
                let count = map_a.get(val).unwrap();
                let updated = count + 1;
                map_a.insert(*val,updated);
            } else {
                map_a.insert(*val, 1);
            }
        }

        let mut set_a: HashSet<i32> = HashSet::new();
        let count_list: Vec<&i32> = map_a.values().into_iter().collect();
        for i in 0..(count_list.len()) {
            let val = count_list.get(i).unwrap();
            if set_a.contains(*val) {
                return false;
            } else {
                set_a.insert(**val);
            }
        }

        return true;
    }
}

fn main() {
    // Example 1
    let arr = vec![1, 2, 2, 1, 1, 3];

    let result = Solution::unique_occurrences(arr);

    println!("Result: {:?}", result);
    println!("Expected: true");

    // Example 2
    let arr = vec![1, 2];

    let result = Solution::unique_occurrences(arr);

    println!("Result: {:?}", result);
    println!("Expected: false");

    // Example 3
    let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];

    let result = Solution::unique_occurrences(arr);

    println!("Result: {:?}", result);
    println!("Expected: true");
}