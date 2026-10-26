/*
Problem E - The Backslash Problem

When you are writing programs for a scripting language, you often have to
worry about how special characters are handled. If you want to suppress
special interpretation and have a special character handled literally,
you can escape it by putting a backslash in front of it. In fact, since
backslash is itself a special character, you also have to escape
backslashes if you want them interpreted literally. For example, to get
a shell program to print out '$', you might have to type something like:
echo \$.

Of course, script programmers can't stop at just one level of
interpretation. We can write Perl code that outputs a shell script,
which itself might output instructions for something like sed. Each
level of interpretation will think that backslashes escape only the
character immediately after them. If we want a special character to
make it through several levels of interpretation, we may have to place
more backslashes in front of it.

Write a program to determine how backslashes should be inserted in
order to produce some desired literal string after multiple levels
of interpretation.

We'll assume that all ASCII characters from '!' up to and including
'*' are special. Also, characters from '[' up to and including ']'
are considered special.

If string s needs to be output after one level of interpretation,
we have to start with a string s' that's just like s, but with every
special character in s escaped.

If we want to produce s after two levels of interpretation, we have
to start with a string s'' that is just like s', but with every
special character in s' escaped.


Input

Input consists of up to 100 test cases. Each test case has two lines.

The first line contains an integer 0 <= n <= 8, the number of levels
of interpretation that should be applied.

The second line contains a string s using only printable ASCII
characters, of length 1 to 100.

Test cases will end with the end of file.


Output

For each test case, output a string s' containing appropriate
backslashes to produce the given string s after n levels of
interpretation.


Sample Input

1
this is a 'test'
2
/:-)


Sample Output

this is a \'test\'
/:-\\\)
*/

use std::{io::{self, Read}};

pub struct Solution {}

impl Solution {
    pub fn execute() -> () {
      let mut input: String = String::new();
      io::stdin().read_to_string(&mut input).unwrap();

      let mut lines = input.lines();      
      while let Some(first) = lines.next() {
          let Some(second) = lines.next() else {
              break; // Ignore a final line that has no partner.
          };

          let mut output: Vec<char> = Vec::new();
          let times: usize = first.parse().unwrap();
          for char in second.chars() {
            if char >= '!' && char <= '*' || char >= '[' && char <= ']' {
              
              for _ in 0..((1usize << times) - 1) {
                output.push('\\');
              }
            }
            output.push(char);
          }

          println!("{}", output.iter().collect::<String>());
      }

      return;
    }
}

// rustc main.rs; if ($LASTEXITCODE -eq 0) { Get-Content input.txt | .\main.exe }
fn main() {
    Solution::execute();
}
