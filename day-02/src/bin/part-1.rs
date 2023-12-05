
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    return input
    .lines()
    .filter(|s| return is_game_possible(s))
    .map(|s| return parse_game_id(s).parse::<usize>().unwrap())
    .sum()
}

fn is_game_possible(s: &str) -> bool {
    
    //Define initial numbers
    let mut ball_table = HashMap::new();
    ball_table.insert("red", 12);
    ball_table.insert("green", 13);
    ball_table.insert("blue", 14);


    //Parse game into individual hands
    let hands_list = s.split(':').skip(1).next().unwrap().to_string();
    let hands : Vec<&str>= hands_list.split(';').collect();

    //A hand looks something like " 3 blue, 4 red"
    for hand in hands {

        //e.g. "3 blue"
        let info_list : Vec<&str> = hand.split(',').collect(); 
        
        //an example color_data is something like " 3 blue".
        for color_data in info_list {
            let (n, color)  = color_data.trim().split_once(' ').unwrap();

            //If any values drop below 0, the game could not have been possible.
            let color_val = ball_table.entry(color).or_default();
            if *color_val  - n.parse::<i32>().unwrap() < 0 {
                return false;
            }
        }

    }

    true
}

fn parse_game_id(s: &str) -> String {
    return s
    .split(' ')
    .nth(1)
    .expect("String only contains one word")
    .trim_end_matches(':')
    .to_string()
}







#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn parse_game_id_test() {
        let test_input = String::from(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        
        let output : Vec<String>  = test_input
        .lines()
        .map(|s| return parse_game_id(s))
        .collect();

        assert_eq!(output, ["1","2","3","4","5"]);
    }

    #[test]
    fn is_game_possible_test() {
        let test_input = String::from(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        
        let output: Vec<bool> = test_input
        .lines()
        .map(|s| return is_game_possible(s))
        .collect();

        assert_eq!(output, [true,true,false,false,true]);
    }

    #[test]
    fn part1_test() {
        let test_input = String::from(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        
        let output = part1(&test_input);

        assert_eq!(output, 8);
    }
}
