
/*
As a homeowner, you tend to experience many problems: Water pipes can
burst, fuses can blow, and—most annoyingly—your chimney can fall over.

Bamse has been particularly suffering from the last problem, and is
tired of constantly having to get the ladder so he can climb up on
the roof.

Since slamming the door is the most frequent cause of fallen chimneys,
Bamse has painstakingly measured and noted every time he closes his
door. His notes show how hard he closed the door in mBp (milli Bamse
power), and whether or not his chimney fell over as a result.

Now, Bamse has just returned from a particularly bad day. He would
like to know how hard he can slam his door without making it even
worse.

How hard can he slam the door, and be guaranteed not to make his
chimney fall over?

INPUT:
The first line contains a single integer n, the number of notes
that Bamse has written.

The next n lines each contain:
- An integer representing the force in mBp.
- A word, either "ja" (yes) or "nej" (no), indicating whether
  the chimney fell over.

It is guaranteed that the notes contain at least one "ja" and
at least one "nej".

OUTPUT:
Print a single integer representing the maximum force in mBp
with which Bamse can slam his door without making the chimney
fall over.

SAMPLE INPUT 1:
5
901 ja
123 nej
666 ja
500 nej
612 nej

SAMPLE OUTPUT 1:
612

SAMPLE INPUT 2:
2
1000 ja
1 nej

SAMPLE OUTPUT 2:
1
*/

use std::{io::{self, Read}};

pub struct Solution {}

impl Solution {
    pub fn execute() -> () {
      let mut input: String = String::new();
      io::stdin().read_to_string(&mut input).unwrap();

      let mut max: i32 = 0;
      for line in input.lines().skip(1) {
        let mut parts = line.split_whitespace();
        let number: i32 = parts.next().unwrap().parse().unwrap();
        let word = parts.next().unwrap();

        if word == "ja" {
          continue;
        }

        if number > max {
          max = number;
        }
      }

      print!("{}", max);

      return;
    }
}

// rustc main.rs; if ($LASTEXITCODE -eq 0) { Get-Content input.txt | .\main.exe }
fn main() {
    Solution::execute();
}