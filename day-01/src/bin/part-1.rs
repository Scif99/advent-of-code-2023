fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i128 {
    return input
    .lines()
    .map(|l| return l.to_string())
    .map(|s|  find_and_combine_digits(&s))
    .sum();
}


fn find_and_combine_digits(s : &String) -> i128 {
    let mut digit_string = String::with_capacity(2);

    let first_digit_index  = s.chars().position(|c| return c.is_digit(10)).expect("Could not find an integer!");
    digit_string.push((s.as_bytes()[first_digit_index]) as char);

    let last_digit_index = (s.chars().count() - 1) -   s.chars().rev().position(|c| return c.is_digit(10)).expect("Could not find an integer!");
    digit_string.push((s.as_bytes()[last_digit_index]) as char);

    return digit_string.parse::<i128>().unwrap();
}





#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn part1_test() {
        let test_input = String::from(
        "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet");
        let output = part1(&test_input);
        assert_eq!(output, 142);
    }
}