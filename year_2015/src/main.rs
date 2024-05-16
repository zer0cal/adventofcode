use std::fs;

mod day1;
mod day2;

fn main() {
    let d1input = fs::read_to_string("day1_input.txt").expect("err reading day 1 input");
    let d1output = day1::what_flor(d1input);
    println!("d1 output: {}", d1output);
    let d1p2input = fs::read_to_string("day1_input.txt").expect("err reading day 1 input");
    let d1p2output = day1::position(d1p2input);
    println!("d1 pt 2 output: {}", d1p2output);

    let d2input = fs::read_to_string("day2_input.txt").expect("err reading day 2 input");
    let mut d2answer = 0;
    let mut d2pt2answer = 0;
    for line in d2input.split_whitespace() {
        let splt: std::str::Split<'_, &str> = line.split("x");
        let v: Vec<i32> = splt.map(|x| x.parse::<i32>().unwrap()).collect();
        let l = v.get(0).unwrap();
        let w = v.get(1).unwrap();
        let h = v.get(2).unwrap();
        d2answer += day2::square_feet_of_wrapping_paper(l, w, h);
        d2pt2answer += day2::feet_of_ribbon(l, w, h);
    }
    println!("d2 answer is: {}", d2answer);
    println!("d2 pt2 answer is: {}", d2pt2answer);
}
