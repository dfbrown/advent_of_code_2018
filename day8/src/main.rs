use std::iter::Iterator;
use parselib::GenericError;

fn compute_sum_metadata<Iter>(iter: &mut Iter) -> Result<usize, GenericError>
where
    Iter: Iterator<Item=usize>
{
    let num_children = iter.next().ok_or(GenericError::new("Could not get num children"))?;
    let num_metadata = iter.next().ok_or(GenericError::new("Could not get num metadata"))?;
    let mut sum_metadata: usize = 0;
    for _ in 0..num_children {
        sum_metadata += compute_sum_metadata(iter)?;
    }
    sum_metadata += iter.take(num_metadata).sum::<usize>();
    return Ok(sum_metadata);
}

fn compute_node_value<Iter>(iter: &mut Iter) -> Result<usize, GenericError>
where
    Iter: Iterator<Item=usize>
{
    let num_children = iter.next().ok_or(GenericError::new("Could not get num children"))?;
    let num_metadata = iter.next().ok_or(GenericError::new("Could not get num metadata"))?;
    let mut child_values = Vec::with_capacity(num_children);
    for _ in 0..num_children {
        child_values.push(compute_node_value(iter)?);
    }
    let node_value = if num_children == 0 {
        iter.take(num_metadata).sum::<usize>()
    } else {
        let mut value = 0;
        for m in iter.take(num_metadata) {
            if m - 1 < num_children {
                value += child_values[m - 1];
            }
        }
        value
    };
    return Ok(node_value);
}

fn main() -> Result<(), Box<std::error::Error>> {
    let mut input: Vec<usize> = Vec::new();
    for num_str in parselib::load_text_file("input.txt")?.split(' ') {
        input.push(num_str.parse()?);
    }
    println!("part1: {}", compute_sum_metadata(&mut input.iter().cloned())?);
    println!("part2: {}", compute_node_value(&mut input.iter().cloned())?);
    Ok(())
}
