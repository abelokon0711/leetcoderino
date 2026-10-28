/*
Problem F - Continuous Median

Given an array A of N integers, calculate the sum of
the medians after inserting each element one by one.

For every prefix A[0..i]:
- If the number of elements is odd, the median is
  the middle element of the sorted prefix.
- If the number of elements is even, the median is
  the average of the two middle elements, rounded down.

Return the sum of all prefix medians.

Input:
- T: Number of test cases (1 <= T <= 100).
- N: Number of elements (1 <= N <= 100000).
- A: N non-negative integers.
- Total elements across all test cases <= 400000.

Output:
- For each test case, print the sum of all medians.

Example:
Input:
2
6
1 3 6 2 7 8
7
1 3 6 2 7 8 5

Output:
15
20

Explanation (first test case):
[1]             -> median = 1
[1, 3]          -> median = 2
[1, 3, 6]       -> median = 3
[1, 2, 3, 6]    -> median = 2
[1, 2, 3, 6, 7] -> median = 3
[1, 2, 3, 6, 7, 8] -> median = 4

Sum = 1 + 2 + 3 + 2 + 3 + 4 = 15
*/

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::{io::{self, Read}};
pub struct Solution {}

impl Solution {
    pub fn execute() -> () {
      let mut input: String = String::new();
      io::stdin().read_to_string(&mut input).unwrap();

      let mut lines = input.lines();
      let example_count: usize = lines.next().unwrap().parse().unwrap();
      for _ in 0..example_count {
        let _ = lines.next().unwrap();
        let array_string: &str = lines.next().unwrap();
        let array: Vec<i64> = array_string
                                .split_whitespace()
                                .map(|x| x.parse::<i64>().unwrap())
                                .collect();

        let mut lower = BinaryHeap::<i64>::new();
        let mut upper = BinaryHeap::<Reverse<i64>>::new();
      
        let mut total: i64 = 0;
        for val in array {
          if lower.peek().map_or(true, |&x| val <= x) {
              lower.push(val);
          } else {
              upper.push(Reverse(val));
          }

          if lower.len() > upper.len() + 1 {
              upper.push(Reverse(lower.pop().unwrap()));
          } else if upper.len() > lower.len() {
              lower.push(upper.pop().unwrap().0);
          }

          let median = if lower.len() == upper.len() {
              (lower.peek().unwrap() + upper.peek().unwrap().0) / 2
          } else {
              *lower.peek().unwrap()
          };

          total += median;
        }
      
        println!("{total}");
      }

      return;
    }
}

// rustc main.rs; if ($LASTEXITCODE -eq 0) { Get-Content input.txt | .\main.exe }
fn main() {
    Solution::execute();
}
