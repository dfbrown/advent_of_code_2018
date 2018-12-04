fn part1(input: &[u8]) -> i64 {
    let mut num_twos = 0;
    let mut num_threes = 0;
    for line in input.chunks(27) {
        let mut num_seen = [0u8; 26];
        assert!(line[26] == '\n' as u8);
        for &v in &line[0..26] {
            assert!(v >= 'a' as u8);
            assert!(v <= 'z' as u8);
            num_seen[(v - 'a' as u8) as usize] += 1;
        }
        if num_seen.iter().any(|&x| x == 2) {
            num_twos += 1;
        }
        if num_seen.iter().any(|&x| x == 3) {
            num_threes += 1;
        }
    }
    return num_twos * num_threes;
}

fn part2(input: &[u8]) -> String {
    let mut iter_line1 = input.chunks(27);
    let mut result = String::with_capacity(26);
    while let Some(line1) = iter_line1.next() {
        let mut iter_line2 = iter_line1.clone();
        while let Some(line2) = iter_line2.next() {
            result.clear();
            for (&letter1, &letter2) in line1[0..26].iter().zip(line2[0..26].iter()) {
                if letter1 == letter2 {
                    result.push(letter1 as char);
                }
            }
            if result.len() == 25 {
                return result;
            }
        }
    }

    panic!("No match found");
}

fn main() {
    // Assume input is unix ascii/utf8 formatted text file with no byte-order-mark.  That way we
    // can interpret the file as bytes where each line is 27 bytes long with the first 26 bytes
    // are ascii characters between 'a' and 'z', and the 27th byte is a unix newline
    let input = parselib::load_bytes("input.txt").expect("Couldn't open input");
    assert!(input.len() % 27 == 0);
    println!("part1: {}", part1(input.as_slice()));
    println!("part2: {}", part2(input.as_slice()));
}
