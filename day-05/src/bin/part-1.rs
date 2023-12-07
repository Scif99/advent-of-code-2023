use std::{collections::{btree_map::Range, HashMap}, ops::RangeInclusive};



fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}


fn part1(input: &str) -> usize {

    let chunks: Vec<&str> =  input
    .split("\n\n")
    .collect();

    let seeds = parse_into_seeds(chunks[0]);

    let maps: Vec<RangeMap>  = chunks[1..]
    .iter()
    .map(|c| parse_into_map_info(c))
    .collect();

    let mut table: HashMap<&str, &RangeMap> = HashMap::new();
    for rmap in &maps {
        table.insert(&rmap.source_name, rmap );
    }


    let lowest = seeds
    .iter()
    .map(|s|{println!("\n\nseed {}", s);   return table.get("seed")       .unwrap().compute_dest_value(*s)})
    .map(|s| {println!("soil {}", s);       return table.get("soil")       .unwrap().compute_dest_value(s)})
    .map(|s| {println!("fertilizer {}", s); return table.get("fertilizer") .unwrap().compute_dest_value(s)})
    .map(|s| {println!("water {}", s);      return table.get("water")      .unwrap().compute_dest_value(s)})
    .map(|s| {println!("light {}", s);      return table.get("light")      .unwrap().compute_dest_value(s)})
    .map(|s| {println!("temperature {}", s);return table.get("temperature").unwrap().compute_dest_value(s)})
    .map(|s| {println!("humidity {}", s);   return table.get("humidity")   .unwrap().compute_dest_value(s)})
    .fold(usize::MAX, |a, b| a.min(b));

    return lowest;

}

fn parse_into_seeds(line: &str) -> Vec<usize> {

    let stripped = line.strip_prefix(&line[0..=line.find(':').unwrap()]).unwrap();
    let seeds: Vec<usize>= stripped.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
    return seeds;
}

fn parse_into_map_info(chunk: &str) -> RangeMap {
    let lines: Vec<&str> = chunk.split("\n").collect();
    let (names_str, data) = (lines[0], lines[1..].to_vec());


    let (source_name, dest_name) = parse_map_names(names_str);
    let ranges = parse_map_values(data); 

    return RangeMap::new(source_name, dest_name,ranges);
}

fn parse_map_names(name_str: &str) -> (&str, &str) {
    let names: Vec<&str>  =name_str
    .strip_suffix(" map:").unwrap()
    .split("-to-")
    .collect();

    return (names[0], names[1]);
}

fn parse_map_values(data_str: Vec<&str>) -> Vec<(usize, usize, usize)> {
    let mut v: Vec<(usize, usize, usize)> = Vec::new();

    for l in data_str.iter() {
        let nums: Vec<usize> = l.split(' ').map(|s| s.parse::<usize>().unwrap()).collect();
        assert!(nums.len() ==3);
        v.push((nums[0],nums[1],nums[2])); //(dest_start, source_start, range_size)
    }
    return v;
}

#[derive(Debug)]
struct RangeMap {

    source_name: String,
    destination_name: String,

    ranges: Vec<(RangeInclusive<usize>, usize)>,
}

impl RangeMap {
    fn new(src_name: &str, dest_name: &str, info: Vec<(usize, usize, usize)>) -> Self{

        let mut r: Vec<(RangeInclusive<usize>, usize)> = Vec::new();
        for (dest_start, src_start, range) in info {
            r.push((src_start..=src_start+range, dest_start))
        }

        Self {
            source_name: src_name.to_string(),
            destination_name: dest_name.to_string(),
            ranges: r
        }
    }
}

impl RangeMap {
    fn compute_dest_value(&self, in_val: usize) -> usize {

        for (srange, dstart) in &self.ranges {
            if srange.contains(&in_val) {
                return dstart + srange.clone().position(|x| return x==in_val).unwrap();
            }
        }

        //If the number isn't mapped then the destination value is just the source value.
        return in_val;
    }
}




#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn part1_test() {
        let test_input: &str  = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let output = part1(&test_input);

        assert_eq!(output, 35);
    }

    #[test]
    fn parse_into_seeds_test() {
        let test_input: &str  = "seeds: 79 14 55 13";
        let output = parse_into_seeds(&test_input);

        assert_eq!(output, [79,14,55,13]);
    }

    #[test]
    fn parse_map_names_test() {
        let test_input: &str  = "seed-to-soil map:";
        let (src,dest) = parse_map_names(&test_input);

        assert_eq!(src, "seed");
        assert_eq!(dest, "soil");
    }

    #[test]
    fn parse_into_map_info_test() {
        let test_input: &str  = "seed-to-soil map:
50 98 2
52 50 48";
        let x = parse_into_map_info(test_input);

        assert_eq!(1, 2);
    }
}