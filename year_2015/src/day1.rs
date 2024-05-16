// --- Day 1: Not Quite Lisp ---
// Santa is trying to deliver presents in a large apartment building, but he can't find the right floor - the
// directions he got are a little confusing. He starts on the ground floor (floor 0) and then follows the
// instructions one character at a time.
// An opening parenthesis, (, means he should go up one floor, and a closing parenthesis, ), means he should go down
// one floor.
// To what floor do the instructions take Santa?

pub fn what_flor(s: String) -> i32 {
    s.into_bytes()
        .iter()
        .map(|&u| match u {
            40 => 1,
            41 => -1,
            _ => 0,
        })
        .reduce(|acc, e| acc + e)
        .unwrap_or_default()
}

// --- Part Two ---
// Now, given the same instructions, find the position of the first character that causes him to enter the basement
// (floor -1). The first character in the instructions has position 1, the second character has position 2, and so
// on.
// What is the position of the character that causes Santa to first enter the basement?

pub fn position(s: String) -> i32 {
    let mut acc = 0;
    for (i, u) in s.into_bytes().iter().enumerate() {
        acc += match u {
            40 => 1,
            41 => -1,
            _ => 0,
        };
        if acc == -1 {
            return (i + 1) as i32;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::position;
    use super::what_flor;

    #[test]
    fn first() {
        let result = what_flor(String::from("(())"));
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn second() {
        let result = what_flor(String::from("()()"));
        let expected = 0;
        assert_eq!(result, expected);
    }

    #[test]
    fn third() {
        let result = what_flor(String::from("((("));
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn forth() {
        let result = what_flor(String::from("(()()("));
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn fifth() {
        let result = what_flor(String::from(")))"));
        let expected = -3;
        assert_eq!(result, expected);
    }

    #[test]
    fn pt2_first() {
        let res = position(String::from(")"));
        let exp = 1;
        assert_eq!(res, exp);
    }

    #[test]
    fn pt2_second() {
        let res = position(String::from("()())"));
        let exp = 5;
        assert_eq!(res, exp);
    }
}
