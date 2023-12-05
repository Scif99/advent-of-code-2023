
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> usize {
    return input
    .lines()
    .map(|s: &str| return minimum_set(s))
    .map(|set:(usize,usize,usize)| return power_of_cubes(set))
    .sum();
}


fn minimum_set(s: &str) -> (usize,usize,usize) {

    //Create a table that stores the largest number of balls needed for each color.
    let mut ball_table: HashMap<&str, usize> = HashMap::new();
    ball_table.insert("red", 0);
    ball_table.insert("green", 0);
    ball_table.insert("blue", 0);


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

            //Update hash table entry if it is larger
            let color_val = ball_table.entry(color).or_default();
            if n.parse::<usize>().unwrap() > *color_val  {
                *color_val = n.parse::<usize>().unwrap();
            }
        }

    }

    return (ball_table.get("blue").unwrap().clone(), ball_table.get("red").unwrap().clone(), ball_table.get("green").unwrap().clone() );
}

fn power_of_cubes(set: (usize,usize,usize)) -> usize {
    return  set.0 * set.1 * set.2;
}







#[cfg(test)]
mod tests {
    use super::*; 
    #[test]
    fn part2_test() {
        let test_input = String::from(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        
        let output = part2(&test_input);

        assert_eq!(output, 2286);
    }
}
