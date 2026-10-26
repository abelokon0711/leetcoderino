/*
Problem D - Tree Skiing
=======================

Description
-----------
After a long drive, you and your friend have arrived
at the ski slope. There is a ton of new snow, the sun
is shining, and you are both feeling adventurous!

When you get off the lift, you see that a new run
has been opened through the trees.

Strangely, the trees in this forest have grown in
a perfectly compass-aligned grid.

Your friend heads off down the slope weaving her
way around the trees. Eventually, she stops in a
clearing to make a snow angel.

You want to join her to make a snow angel as well,
but you do not want to waste the opportunity to ski
on the fresh powder. You go looking for a new path
through the forest.

You cannot ski straight down the mountain to your
friend because there are trees in the way.

At each clearing, you have an option to head directly
north or directly west out of the clearing.

You do not want to climb back uphill to the south
or east.

Once you pass the clearing and enter the next,
you have to make the same decision again and again.

How many different ways can you ski through the trees
until you reach the clearing where your friend is
playing in the snow, without skiing on exactly the
same path as she skied and without hitting a tree
on your way down?

Input
-----
The first line contains an integer n:

    1 <= n <= 20

The second line contains n characters separated
by spaces which specify, in order, the directions
your friend went at each clearing.

The characters are limited to:

    N = North
    W = West

Output
------
Print a single integer: the number of other possible
paths you could take to meet up with your friend.

Do not count the exact path your friend took.

Sample Input 1
--------------
3
N N W

Sample Output 1
---------------
2

Sample Input 2
--------------
4
N W N W

Sample Output 2
---------------
5
*/

use std::{io::{self, Read}};

pub struct Solution {}

impl Solution {
    pub fn execute() -> () {
      let mut input: String = String::new();
      io::stdin().read_to_string(&mut input).unwrap();

      let mut n_count: u64 = 0;
      let mut w_count: u64 = 0;

      let line = input.lines().skip(1).next();
      let parts = line.unwrap().split_whitespace();
      for direction in parts {
        if direction == "N" {
          n_count += 1;
          continue;
        }

        if direction == "W" {
          w_count += 1;
        }
      }

      let total: u64 = w_count + n_count;
      let numerator: u64 = (1..=total).product();
      let denominator: u64 = (1..=n_count).product::<u64>()
                              * (1..=w_count).product::<u64>();

      print!("{}", (numerator / denominator) - 1);
      return;
    }
}

// rustc main.rs; if ($LASTEXITCODE -eq 0) { Get-Content input.txt | .\main.exe }
fn main() {
    Solution::execute();
}
