use std::collections::HashSet;

fn part2(values: &[i64]) -> i64 {
    let mut seen_values = HashSet::new();
    let mut freq = 0;
    seen_values.insert(freq);
    for v in values.iter().cycle() {
        freq += v;
        if !seen_values.insert(freq) {
            return freq;
        }
    }
    panic!("infinite loop");
}

fn main() {
    let input = parselib::parse_lines::<i64, _>("input.txt").expect("Parse error");
    println!("part 1: {}", input.iter().sum::<i64>());
    println!("part 2: {}", part2(input.as_slice()));
}
