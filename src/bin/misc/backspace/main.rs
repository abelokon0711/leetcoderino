/*
Problem B - Backspace
=====================

Submissions to this problem will only be marked as accepted
if they receive a score of 100.

Description
-----------
Right before Forritunarkeppni Framhaldsskólanna started,
Bjarki decided to update his computer.

He didn't notice anything wrong until he started writing
code in his favourite editor Bim (Bjarki IMproved).

Usually, pressing backspace erases one letter to the left
of the cursor. But after the update, pressing backspace
instead writes the symbol '<'.

All his other editors have the same issue.

Help Bjarki write a program that takes as input a string
he wrote and prints the string as he intended to write it.

Input
-----
The first and only line contains a string S of length N,
consisting only of lowercase English letters and '<'.

Constraints:
    1 <= N <= 10^6

Output
------
Print the string as Bjarki intended to write it.

Process the input character by character:
- A lowercase letter is added to the output.
- The symbol '<' removes the last added letter,
  acting as a backspace.

Sample Input 1
--------------
a<bc<

Sample Output 1
---------------
b

Sample Input 2
--------------
foss<<rritun

Sample Output 2
---------------
forritun

Sample Input 3
--------------
a<a<a<aa<<

Sample Output 3
---------------
(empty string)

Explanation
-----------
Sample 1:
Bjarki writes 'a', erases it, writes 'b' and 'c',
then erases 'c'. The result is "b".

Sample 2:
Bjarki writes "foss", then erases the last two
letters with "<<", leaving "fo".
He then writes "rritun", producing "forritun".

Sample 3:
Bjarki writes 'a' and immediately erases it
three times. He then writes "aa" and erases
both letters with "<<". The result is empty.

Scoring
-------
Group 1: 10 points
    1 <= N <= 100
    S only contains the letter 'a' and '<'.

Group 2: 10 points
    1 <= N <= 100
    S does not contain two consecutive '<'.

Group 3: 40 points
    1 <= N <= 100

Group 4: 40 points
    1 <= N <= 10^6

Total: 100 points

*/

use std::{io::{self, Read}};

pub struct Solution {}

impl Solution {
    pub fn execute() -> () {
      let mut input: String = String::new();
      io::stdin().read_to_string(&mut input).unwrap();

      let bjarkis_text: &str = input.lines().next().unwrap();
      let mut output: Vec<char> = vec!['a'; bjarkis_text.len()];
      let mut index: usize = 0;
      for char in bjarkis_text.chars() {
        if char == '<' {
          if index > 1 {
            index -= 1;
          } else {
            index = 0;
          }

          continue;
        }

        output[index] = char;
        index += 1;
      }

      output.truncate(index);

      print!("{}", output.iter().collect::<String>());

      return;
    }
}

// rustc main.rs; if ($LASTEXITCODE -eq 0) { Get-Content input.txt | .\main.exe }
fn main() {
    Solution::execute();
}