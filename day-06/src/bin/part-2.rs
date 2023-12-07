fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> usize {

    let lines: Vec<&str>     = input.lines().collect();
    let times_str      = lines[0];
    let distances_str  = lines[1];

    return num_ways_to_beat(parse_line(times_str), parse_line(distances_str));
}

fn parse_line(s: &str) -> usize {
    let stripped = s.strip_prefix(&s[0..=s.find(':').unwrap()]).unwrap();
    let num_str: Vec<&str> = stripped.split_whitespace().collect();
    return num_str
    .iter()
    .fold(String::new(), |acc,&b| acc + b)
    .parse::<usize>().unwrap();
}

fn num_ways_to_beat(time: usize, record_distance: usize) -> usize {

    let mut n = 0;
    for i in 0..=time {
        let boat_speed = i;
        let time_remaining = time - i;

        let distance_travelled = boat_speed * time_remaining;
        if distance_travelled > record_distance {
            n+=1;
        }
    }
    return n;
}






#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn parse_line_test() {
        let test_input: &str = "Time:        44     70     70     80";

        let output = parse_line(&test_input);
        assert_eq!(output, 44707080);
    }
}