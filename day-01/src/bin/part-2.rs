use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i128 {
    return input
    .lines()
    .map(|l| return l.to_string())
    .map(|s|  find_and_combine_digits_or_words(&s))
    .sum();
}

//This time instead of looking for the first position of a digit,
//we look for the first occurence of a digit or a substring in our table.
fn find_and_combine_digits_or_words(s : &String) -> i128 {

    //TODO: make this static?
    let digit_strings : Vec<String> = vec![
        String::from("one"), 
        String::from("two"  ),
        String::from("three"),
        String::from("four" ),
        String::from("five" ),
        String::from("six"  ),
        String::from("seven"),
        String::from("eight"),
        String::from("nine" ),
        String::from("1" ),
        String::from("2" ),
        String::from("3" ),
        String::from("4" ),
        String::from("5" ),
        String::from("6" ),
        String::from("7" ),
        String::from("8" ),
        String::from("9" )
        ];

    let first_digit  = find_first_digit_char(&s, &digit_strings);
    let last_digit   = find_last_digit_char(&s, &digit_strings);

    let mut combined_number = String::with_capacity(2);
    combined_number.push(first_digit);
    combined_number.push(last_digit);

    return combined_number.parse::<i128>().unwrap();
}

/// Searches a string for matches stored in a list, and returns the index of the first character of the matching substring if found.
/// @param main_string: String to search through.
/// @param strings_to_find: List of strings to check for matches. 
/// 
fn find_first_digit_char(main_string: &str, strings_to_find: &[String]) -> char {

    //First value is the digit as a char.
    //Second value is the index into main_string of the first char of the pattern
    let mut first : Option<(char, usize)> = None;

    //Iterate over all patterns to search for.
    //If we find a match, update first_word.
    for val in strings_to_find.iter() {
        if let Some(first_char_idx) = main_string.find(val) {
            match first {
                None => first = Some((digit_string_to_char(val),first_char_idx)), //First match
                Some((_,j)) => {
                    //Only update the digit if the index is lower.
                    if first_char_idx < j {
                        first = Some((digit_string_to_char(val), first_char_idx));
                    }
                }
            }
        }
    }
    return first.unwrap().0;
    
}


fn find_last_digit_char(main_string: &str, strings_to_find: &[String]) -> char {

    //First value is the digit as a char.
    //Second value is the index into main_string of the first char of the pattern
    let mut last : Option<(char, usize)> = None;

    //Iterate over all patterns to search for.
    //If we find a match, update first_word.
    for val in strings_to_find.iter() {
        if let Some(last_char_idx) = main_string.rfind(val) {
            match last {
                None => last = Some((digit_string_to_char(val),last_char_idx)), //First match
                Some((_,j)) => {
                    //Only update the digit if the index is lower.
                    if last_char_idx > j {
                        last = Some((digit_string_to_char(val), last_char_idx));
                    }
                }
            }
        }
    }
    return last.unwrap().0;
}

//Helper function that converts 'wordified' digit into a char of the associated number.
fn digit_string_to_char( ds: &String) -> char {

    let mut word_table: HashMap<String, char> = HashMap::new();
    word_table.insert(String::from("one"  ), char::from('1'));
    word_table.insert(String::from("two"  ), char::from('2'));
    word_table.insert(String::from("three"), char::from('3'));
    word_table.insert(String::from("four" ), char::from('4'));
    word_table.insert(String::from("five" ), char::from('5'));
    word_table.insert(String::from("six"  ), char::from('6'));
    word_table.insert(String::from("seven"), char::from('7'));
    word_table.insert(String::from("eight"), char::from('8'));
    word_table.insert(String::from("nine" ), char::from('9'));

    // "1"   -> "1"
    // "one" -> "1"
    match ds.len() {
        1 => return ds.chars().next().unwrap(),
        _ => return word_table.get(ds).unwrap().clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn part1_test() {
        let test_input = String::from(
        "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen");
        let output = part2(&test_input);
        assert_eq!(output, 281);
    }
}