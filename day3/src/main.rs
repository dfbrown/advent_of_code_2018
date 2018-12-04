use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Claim {
    col: usize,
    row: usize,
    width: usize,
    height: usize,
}

impl FromStr for Claim {
    type Err = parselib::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r"\d+ @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        };

        let captures = RE.captures(s).ok_or(parselib::ParseError)?;
        return Ok(Claim {
            col: captures.get(1).unwrap().as_str().parse().unwrap(),
            row: captures.get(2).unwrap().as_str().parse().unwrap(),
            width: captures.get(3).unwrap().as_str().parse().unwrap(),
            height: captures.get(4).unwrap().as_str().parse().unwrap(),
        });
    }
}

fn make_filled_grid(input: &[Claim]) -> HashMap<(usize, usize), usize> {
    let mut squares: HashMap<(usize, usize), usize> = HashMap::new();

    for claim in input {
        for col in claim.col..claim.col + claim.width {
            for row in claim.row..claim.row + claim.height {
                *squares.entry((row, col)).or_insert(0) += 1;
            }
        }
    }

    return squares;
}

fn part1(grid: &HashMap<(usize, usize), usize>) -> usize {
    return grid
        .iter()
        .fold(0, |acc, (&_, &v)| if v > 1 { acc + 1 } else { acc });
}

fn part2(claims: &[Claim], grid: &HashMap<(usize, usize), usize>) -> usize {
    'claim: for (i, claim) in claims.iter().enumerate() {
        for col in claim.col..claim.col + claim.width {
            for row in claim.row..claim.row + claim.height {
                if grid[&(row, col)] > 1 {
                    continue 'claim;
                }
            }
        }
        return i + 1;
    }
    panic!("No non-overlapping claim");
}

fn main() {
    let input = parselib::parse_lines::<Claim, _>("input.txt").expect("Could not parse input");
    let grid = make_filled_grid(input.as_slice());
    println!("part 1: {}", part1(&grid));
    println!("part 1: {}", part2(input.as_slice(), &grid));
}
