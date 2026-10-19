/*
Problem C - Best Relay Team
===========================

Description
-----------
You are the coach of the national athletics team and need
to select four sprinters for the 4 x 100 m relay.

The relay consists of 4 legs, each 100 meters long.

In the 1st leg, the runner starts normally.

In the 2nd, 3rd, and 4th legs, runners have a flying
start because they are already running when receiving
the baton.

Each runner has two different times:
- a: Time needed for the 1st leg.
- b: Time needed for any other leg.

Each runner can only run one leg.

Your task is to choose four different runners and
assign them to the four legs so that the total
relay time is minimized.

Input
-----
The first line contains an integer N:

    4 <= N <= 500

N is the number of available runners.

The next N lines each contain:

    name a b

Where:
- name is the runner's unique name.
- Names consist of 2 to 20 uppercase English letters.
- a is the time for the 1st leg.
- b is the time for the 2nd, 3rd, or 4th leg.
- 8 <= a < 20
- a - 1 <= b <= a

All times are given in seconds with exactly
two digits after the decimal point.

Output
------
Print the minimum possible total relay time.

The answer must have an absolute or relative
error of at most 10^-9.

Then print the names of the four selected runners,
one per line, in the following order:

1. Runner for the 1st leg.
2. Runner for the 2nd leg.
3. Runner for the 3rd leg.
4. Runner for the 4th leg.

Any optimal team is accepted.

Sample Input 1
--------------
6
ASHMEADE 9.90 8.85
BLAKE 9.69 8.72
BOLT 9.58 8.43
CARTER 9.78 8.93
FRATER 9.88 8.92
POWELL 9.72 8.61

Sample Output 1
---------------
35.54
CARTER
BOLT
POWELL
BLAKE

Sample Input 2
--------------
9
AUSTRIN 15.60 14.92
DRANGE 15.14 14.19
DREGI 15.00 14.99
LAAKSONEN 16.39 14.97
LUNDSTROM 15.83 15.35
MARDELL 13.36 13.20
POLACEK 13.05 12.55
SANNEMO 15.23 14.74
SODERMAN 13.99 12.57

Sample Output 2
---------------
52.670000
MARDELL
POLACEK
SODERMAN
DRANGE
*/

use std::{io::{self, Read}};
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn execute() {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();

        let mut acceleration_map = HashMap::new();
        let mut flying_start_map = HashMap::new();

        for line in input.lines().skip(1) {
            let mut parts = line.split_whitespace();

            let name = parts.next().unwrap();

            let acceleration_time: f64 =
                parts.next().unwrap().parse().unwrap();

            let flying_start_time: f64 =
                parts.next().unwrap().parse().unwrap();

            acceleration_map.insert(name, acceleration_time);
            flying_start_map.insert(name, flying_start_time);
        }

        let mut sorted_flying_start_list: Vec<_> =
            flying_start_map.keys().collect();

        sorted_flying_start_list.sort_by(|a, b| {
            flying_start_map[*a].total_cmp(&flying_start_map[*b])
        });

        let mut best_sum = f64::INFINITY;
        let mut best_names = Vec::new();

        for &first_name in acceleration_map.keys() {
            let mut sum = acceleration_map[first_name];
            let mut names = vec![first_name];

            for &name in sorted_flying_start_list.iter().take(4) {
                // A runner cannot run twice
                if *name == first_name {
                    continue;
                }

                sum += flying_start_map[name];
                names.push(name);

                if names.len() == 4 {
                    break;
                }
            }

            if sum < best_sum {
                best_sum = sum;
                best_names = names;
            }
        }

        println!("{:.10}", best_sum);

        for name in best_names {
            println!("{}", name);
        }
    }
}

// rustc main.rs; if ($LASTEXITCODE -eq 0) { Get-Content input.txt | .\main.exe }
fn main() {
    Solution::execute();
}