// --- Day 5: Doesn't He Have Intern-Elves For This? ---

#![allow(dead_code)]

use std::fs;

pub fn print() {
    println!("Day 5: Doesn't He Have Intern-Elves For This?");
    let input = fs::read_to_string("day5_input.txt").expect("err reading day 5 input");
    let ans = input.split_whitespace().filter(|x| nice_string(x)).count();
    let ans2 = input
        .split_whitespace()
        .filter(|x| better_nice_string(x))
        .count();
    println!("answer for pt1 is {ans}");
    println!("answer for pt2 is {ans2}");
}

// Santa needs help figuring out which strings in his text file are naughty or nice.

// A nice string is one with all of the following properties:

// It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
// It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc,
// or dd).
// It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.

// For example:
// ugknbfddgicrmopn is nice because it has at least three vowels (u...i...o...), a double letter (...dd...), and
// none of the disallowed substrings.
// aaa is nice because it has at least three vowels and a double letter, even though the letters used by
// different rules overlap.
// jchzalrnumimnmhp is naughty because it has no double letter.
// haegwjzuvuyypxyu is naughty because it contains the string xy.
// dvszwmarrgswjxmb is naughty because it contains only one vowel.

// How many strings are nice?

fn nice_string(s: &str) -> bool {
    if s.as_bytes()
        .iter()
        .filter(|x| [b'a', b'e', b'i', b'o', b'u'].contains(x))
        .count()
        < 3
    {
        return false;
    }
    if !s.as_bytes().windows(2).any(|x| x[0] == x[1]) {
        return false;
    }
    if !s
        .as_bytes()
        .windows(2)
        .any(|x| [(b'a', b'b'), (b'c', b'd'), (b'p', b'q'), (b'x', b'y')].contains(&(x[0], x[1])))
    {
        return true;
    }
    false
}

// --- Part Two ---
// Realizing the error of his ways, Santa has switched to a better model of determining whether a string is
// naughty or nice. None of the old rules apply, as they are all clearly ridiculous.

// Now, a nice string is one with all of the following properties:

// It contains a pair of any two letters that appears at least twice in the string without overlapping, like
// xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
// It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi
// (efe), or even aaa.

// For example:
// qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj) and a letter that repeats with exactly
// one letter between them (zxz).
// xxyxx is nice because it has a pair that appears twice and a letter that repeats with one between, even
// though the letters used by each rule overlap.
// uurcxstgmygtbstg is naughty because it has a pair (tg) but no repeat with a single letter between them.
// ieodomkazucvgmuy is naughty because it has a repeating letter with one between (odo), but no pair that
// appears twice.

// How many strings are nice under these new rules?

fn better_nice_string(s: &str) -> bool {
    let l = s.len();
    if !s.as_bytes().windows(3).any(|x| x[0] == x[2]) {
        return false;
    }
    for (i, pair) in s[..l - 2].as_bytes().windows(2).enumerate() {
        if s[i + 2..].as_bytes().windows(2).any(|x| x == pair) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::day5::{better_nice_string, nice_string};

    #[test]
    fn part_one() {
        assert!(nice_string("ugknbfddgicrmopn"));
        assert!(nice_string("aaa"));
        assert!(!nice_string("jchzalrnumimnmhp"));
        assert!(!nice_string("haegwjzuvuyypxyu"));
        assert!(!nice_string("dvszwmarrgswjxmb"));
    }

    #[test]
    fn part_two() {
        assert!(better_nice_string("qjhvhtzxzqqjkmpb"));
        assert!(better_nice_string("xxyxx"));
        assert!(!better_nice_string("uurcxstgmygtbstg"));
        assert!(!better_nice_string("ieodomkazucvgmuy"));
    }
}
