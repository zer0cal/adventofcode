// --- Day 8: Matchsticks ---

use std::fs;

pub fn answer() {
    println!("Day 8: Matchsticks");
    let input = fs::read_to_string("day8_input.txt").expect("err reading day 8 input");
    let (characters, codes) = count_total_and_in_mem_chars(&input);
    println!(
        "answer to pt 1 is {} - {} = {}",
        codes,
        characters,
        codes - characters
    );
}

// Space on the sleigh is limited this year, and so Santa will be bringing his list as a digital copy. He needs
// to know how much space it will take up when stored.

// It is common in many programming languages to provide a way to escape special characters in strings. For
// example, C, JavaScript, Perl, Python, and even PHP handle special characters in very similar ways.

// However, it is important to realize the difference between the number of characters in the code
// representation of the string literal and the number of characters in the in-memory string itself.

// For example:

// "" is 2 characters of code (the two double quotes), but the string contains zero characters.
// "abc" is 5 characters of code, but 3 characters in the string data.
// "aaa\"aaa" is 10 characters of code, but the string itself contains six "a" characters and a single, escaped
// quote character, for a total of 7 characters in the string data.
// "\x27" is 6 characters of code, but the string itself contains just one - an apostrophe ('), escaped using
// hexadecimal notation.
// Santa's list is a file that contains many double-quoted string literals, one on each line. The only escape
// sequences used are \\ (which represents a single backslash), \" (which represents a lone double-quote
// character), and \x plus two hexadecimal characters (which represents a single character with that ASCII code)

// Disregarding the whitespace in the file, what is the number of characters of code for string literals minus
// the number of characters in memory for the values of the strings in total for the entire file?

// For example, given the four strings above, the total number of characters of string code (2 + 5 + 10 + 6 = 23
// minus the total number of characters in memory for string values (0 + 3 + 7 + 1 = 11) is 23 - 11 = 12.

fn count_total_and_in_mem_chars(s: &str) -> (u32, u32) {
    let mut characters = 0;
    let mut codes = 0;
    let mut c = s.bytes().into_iter();
    loop {
        match c.next() {
            Some(13) => (),
            Some(10) => (),
            Some(b'\"') => {
                codes += 1;
            }
            Some(b'\\') => {
                codes += 1;
                match c.next() {
                    Some(b'x') => {
                        codes += 3;
                        characters += 1;
                        c.next();
                        c.next();
                    }
                    None => {
                        break;
                    }
                    Some(_) => {
                        codes += 1;
                        characters += 1;
                    }
                }
            }
            None => {
                break;
            }
            Some(_) => {
                codes += 1;
                characters += 1;
            }
        }
    }
    (characters, codes as u32)
}

#[cfg(test)]
mod tests {
    use crate::day8::count_total_and_in_mem_chars;
    use std::fs;

    #[test]
    fn fist_line_from_file() {
        let input =
            fs::read_to_string(r"src\day8_input_first_line.txt").expect("err reading day 8 input");
        println!("{}", input);
        println!("{:?}", input.bytes());
        let (characters, codes) = count_total_and_in_mem_chars(&input);
        assert_eq!((characters, codes), (7, 9));
    }

    #[test]
    fn oneline() {
        let s = r#""aaa""#;
        let (characters, codes) = count_total_and_in_mem_chars(s);
        assert_eq!((characters, codes), (3, 5));
    }

    #[test]
    fn empty() {
        let s = r#"""
""
"""#;
        let (characters, codes) = count_total_and_in_mem_chars(s);
        assert_eq!((characters, codes), (0, 6));
    }

    #[test]
    fn singlebackslash() {
        let s = r#""\\""#;
        let (characters, codes) = count_total_and_in_mem_chars(s);
        assert_eq!((characters, codes), (1, 4));
    }

    #[test]
    fn doublequote() {
        let s = r#""\"""#;
        let (characters, codes) = count_total_and_in_mem_chars(s);
        assert_eq!((characters, codes), (1, 4));
    }

    #[test]
    fn ascii_character() {
        let s = r#""\x11""#;
        let (characters, codes) = count_total_and_in_mem_chars(s);
        assert_eq!((characters, codes), (1, 6));
    }

    #[test]
    fn multiline() {
        let s = r#"""
"abc"
"aaa\"aaa"
"\x27""#;
        let (characters, codes) = count_total_and_in_mem_chars(s);
        assert_eq!((characters, codes), (11, 23));
    }
}
