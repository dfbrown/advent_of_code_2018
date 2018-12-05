use std::cmp;

fn reduced_length(mut input: Vec<u8>) -> usize {
    let mut index = 0;
    while index < input.len() - 1 {
        assert!(input[index] < 128);
        assert!(input[index + 1] < 128);
        if (input[index] as i8 - input[index + 1] as i8).abs() == 32 {
            input.remove(index);
            input.remove(index);
            index = if index == 0 { index } else { index - 1 };
        } else {
            index += 1;
        }
    }
    return input.len();
}

fn part2(input: &[u8]) -> usize {
    let mut min_length = input.len();
    for char_to_remove in 'a' as u8..'z' as u8 {
        let reduced_polymer = input
            .iter()
            .cloned()
            .filter(|x| (x | 0x20) != char_to_remove)
            .collect();
        min_length = cmp::min(min_length, reduced_length(reduced_polymer));
    }
    return min_length;
}

fn main() {
    let input = parselib::load_bytes("input.txt").expect("Failed to parse input");
    println!("part1: {}", reduced_length(input.clone()));
    println!("part2: {}", part2(input.as_slice()));
}
