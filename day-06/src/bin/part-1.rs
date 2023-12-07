use core::iter::zip;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {

    let lines: Vec<&str>     = input.lines().collect();
    let times_str      = lines[0];
    let distances_str  = lines[1];

    return zip(parse_line(times_str), parse_line(distances_str))
    .map(|(t,d)| return num_ways_to_beat(t, d))
    .fold(1, |a,b| return a*b);
}

fn parse_line(s: &str) -> Vec<usize> {
    let stripped = s.strip_prefix(&s[0..=s.find(':').unwrap()]).unwrap();
    return stripped.split_whitespace().map(|c| c.parse::<usize>().unwrap()).collect();
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
        assert_eq!(output, [44,70,70,80]);
    }
}