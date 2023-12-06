use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {

    return input
    .lines()
    .map(|l| return parse_line(l))
    .map(|(w,c)| return points(w,c) )
    .sum();
}


fn parse_line(line: &str)  -> (Vec<usize>, Vec<usize>) {
    let stripped = line
    .strip_prefix(&line[0..=line.find(':').unwrap()]).unwrap();
    
    let (winning, current) = stripped.split_at(stripped.find('|').unwrap());

    let w: Vec<usize> = winning.split_whitespace().filter(|&s| return !s.is_empty()).map(|s| return s.parse::<usize>().unwrap()).collect();
    let c: Vec<usize> = current.split_whitespace().filter(|&s| return !s.is_empty() && s!=String::from("|")).map(|s| return s.parse::<usize>().unwrap()).collect();
    return (w,c);
}



fn points(winning: Vec<usize>, current: Vec<usize>) -> usize{

    let mut table: HashSet<usize> = HashSet::new();

    for w in winning {
         table.insert(w);
    }

    let mut num_matches = 0;
    for c in current {
        if table.contains(&c) {
            num_matches+=1;
        }
    }

    let points = if num_matches > 0  { usize::pow(2,num_matches-1) } else { 0 };
    return points;
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn part1_test() {
        let test_input = String::from(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        let output = part1(&test_input);
        assert_eq!(output, 142);
    }
}

