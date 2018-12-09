use parselib::ParseError;
use std::cmp;

fn parse_line(line: &str) -> Result<(isize, isize), ParseError> {
    let mut iter = line.split(", ");
    let mut next_int = || {
        iter.next()
            .ok_or(ParseError)?
            .parse()
            .map_err(|_| ParseError)
    };

    return Ok((next_int()?, next_int()?));
}

fn find_unique_closest(position: (isize, isize), positions: &[(isize, isize)]) -> Option<usize> {
    let mut min_dist = isize::max_value();
    let mut min_dist_index = 0;
    let mut num_min_dist = 0;
    for (i, (x, y)) in positions.into_iter().enumerate() {
        let dist = (x - position.0).abs() + (y - position.1).abs();
        if dist < min_dist {
            min_dist_index = i;
            min_dist = dist;
            num_min_dist = 1;
        } else if dist == min_dist {
            num_min_dist += 1;
        }
    }
    return if num_min_dist == 1 {
        Some(min_dist_index)
    } else {
        None
    };
}

fn grid_size(positions: &[(isize, isize)]) -> (isize, isize, isize, isize) {
    let mut row_min = isize::max_value();
    let mut row_max = isize::min_value();
    let mut col_min = isize::max_value();
    let mut col_max = isize::min_value();
    for &(row, col) in positions {
        row_min = cmp::min(row, row_min);
        row_max = cmp::max(row, row_max);
        col_min = cmp::min(col, col_min);
        col_max = cmp::max(col, col_max);
    }
    return (row_min, row_max, col_min, col_max);
}

fn part1(positions: &[(isize, isize)]) -> isize {
    let (min_row, max_row, min_col, max_col) = grid_size(positions);
    let mut grid = Vec::with_capacity(((max_row - min_row) * (max_col - min_col)) as usize);

    for row in min_row..max_row {
        for col in min_col..max_col {
            grid.push(find_unique_closest((row, col), positions).map_or(-1, |i| i as i32));
        }
    }

    let mut areas = vec![0isize; positions.len()];
    for &v in grid.iter() {
        if v >= 0 {
            areas[v as usize] += 1;
        }
    }

    let num_cols = (max_col - min_col) as usize;
    for &v in grid.iter().take(num_cols) {
        if v >= 0 {
            areas[v as usize] = 0;
        }
    }
    for &v in grid.iter().rev().take(num_cols) {
        if v >= 0 {
            areas[v as usize] = 0;
        }
    }
    for &v in grid.iter().step_by(num_cols) {
        if v >= 0 {
            areas[v as usize] = 0;
        }
    }
    for &v in grid.iter().skip(num_cols - 1).step_by(num_cols) {
        if v >= 0 {
            areas[v as usize] = 0;
        }
    }

    return *areas.iter().max().unwrap();
}

fn part2(positions: &[(isize, isize)]) -> isize {
    let (min_row, max_row, min_col, max_col) = grid_size(positions);
    let mut result = 0;
    for row in min_row..max_row {
        for col in min_col..max_col {
            let total_dist: isize = positions
                .iter()
                .map(|&(pos_row, pos_col)| (row - pos_row).abs() + (col - pos_col).abs())
                .sum();
            if total_dist < 10000 {
                result += 1;
            }
        }
    }
    return result;
}

fn main() {
    let input = parselib::parse_lines_fn("input.txt", parse_line).expect("Could not parse input");
    println!("part 1: {}", part1(input.as_slice()));
    println!("part 2: {}", part2(input.as_slice()));
}
