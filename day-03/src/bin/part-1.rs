use regex::Regex;


type NumberInfo = (usize,usize,usize, usize); //(line_num, start_index, end_index, value)

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();

    //Sum of all non-adjacent numbers.
    let mut sum = 0;

    //Gather all info we need. 
    let info_list: Vec<NumberInfo> = get_number_info(&lines);

    //Iterate over all the numbers, and add the adjacent ones to the sum.
    for num_info in info_list.iter() {
        if is_number_adjacent_to_symbol(num_info, &lines) {
            sum+=num_info.3;
        }
    }
    return sum;
}

fn get_number_info(lines: &Vec<&str>) -> Vec<NumberInfo> {

    let mut info:  Vec<NumberInfo> = Vec::new(); 

    for (i, val) in lines.iter().enumerate() {
        info.append(&mut get_number_indices(val, i));
    }
    return info;
}

fn get_number_indices(line: &str, line_num: usize) -> Vec<NumberInfo> {
    let regex = Regex::new(r"[0-9]+").expect("Invalid regex pattern");
    let mut indices: Vec<NumberInfo> = Vec::new();

    for mat in regex.find_iter(line) {
        indices.push((line_num, mat.start(), mat.end(), line[mat.start()..mat.end()].parse::<usize>().unwrap()));
    }

    indices
}

fn is_number_adjacent_to_symbol(n_info: &NumberInfo, lines: &Vec<&str>) -> bool {

    let line_num = n_info.0;
    let start_idx = n_info.1;
    let end_idx = n_info.2;

    let char_idx_range = 0..=lines[0].len()-1;
    let line_num_range = 0..=lines.len() - 1;

    //Just checks whether an index lies in bounds
    let closure = |ln: usize, ln_delta: isize, ch: usize, ch_delta: isize| -> Option<u8>{
        //First match is to check if indices ends up < 0.
        match (ln.checked_add_signed(ln_delta), ch.checked_add_signed(ch_delta)) {
            (Some(l), Some(c)) => {
                //Second match is to check that indices are < max size.
                if line_num_range.contains(&l) && char_idx_range.contains(&c)  {
                    return Some(lines[l].as_bytes()[c]);
                }
                else {
                    return None;
                }
            },
            _ => return None
        }
    };

    //Check all adjacent characters
    for i in start_idx..end_idx {
        if  is_symbol(closure(line_num, -1, i, -1)) || //top left adjacent
            is_symbol(closure(line_num, -1, i,  0)) || //top adjacent
            is_symbol(closure(line_num, -1, i,  1)) || //top right adjacent
            is_symbol(closure(line_num,  0, i, -1)) || //left adjacent
            is_symbol(closure(line_num,  0, i,  1)) || //right adjacent
            is_symbol(closure(line_num,  1, i, -1)) || //bottom left adjacent
            is_symbol(closure(line_num,  1, i,  0)) || //bottom adjacent
            is_symbol(closure(line_num,  1, i,  1)) {  //bottom right adjacent
                return true;
            }
    }
    return false;
}


//Option because 
fn is_symbol(c: Option<u8>) -> bool {

    match c {
        Some(x) => {
            let ch = x as char;
            return !ch.is_digit(10) && ch!='.'; 
        }, 
        None => false
    }
}


#[cfg(test)]
mod tests {

    use super::*; 

    #[test]
    fn symbol_test() {
        let s: Vec<Option<u8>> = vec![Some(b'*'), Some(b'+'), Some(b'0'), Some(b'/'), None, ];
        let output: Vec<bool> = s.iter().map(|u| return is_symbol(*u)).collect();

        assert_eq!(output, [true, true, false, true, false]);
    }

    #[test]
    fn get_number_indices_test() {
        let test_input = String::from(
        "467..114..");
        
        let output =  get_number_indices(&test_input, 0);

        assert_eq!(output, [(0,0,3,467),(0,5,8,114)]);
    }

    #[test]
    fn adjacent_test() {
        let test_input = String::from(
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
        
        let lines = test_input.lines().collect();

        let number_info: NumberInfo = (0,5,7,114);

        let output = is_number_adjacent_to_symbol(&number_info, &lines);

        assert_eq!(output, false);
    }

    #[test]
    fn part1_test() {
        let test_input = String::from(
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");

        let output = part1(&test_input);
        assert_eq!(output, 4361);
    }
}
