use regex::Regex;
use std::cell::Cell;
use std::cmp;

fn parse_constraints(s: &str) -> Result<(char, char), parselib::ParseError> {
    lazy_static::lazy_static! {
        static ref CONSTRAINT_RE: Regex = Regex::new(r"Step ([A-Z]).*step ([A-Z])").unwrap();
    };

    let captures = CONSTRAINT_RE.captures(s).ok_or(parselib::ParseError)?;
    return Ok((
        captures.get(1).unwrap().as_str().chars().next().unwrap(),
        captures.get(2).unwrap().as_str().chars().next().unwrap(),
    ));
}

fn make_task_deps(constraints: &[(char, char)]) -> Vec<Vec<u8>> {
    let mut task_deps: Vec<Vec<u8>> = Vec::new();

    for &(dep, task) in constraints {
        assert!(dep >= 'A');
        assert!(task >= 'A');
        let dep_index = dep as u8 - 'A' as u8;
        let task_index = task as u8 - 'A' as u8;
        let max_size = (cmp::max(dep_index, task_index) + 1) as usize;
        if max_size > task_deps.len() {
            task_deps.resize(max_size, Vec::new());
        }
        task_deps[task_index as usize].push(dep_index);
    }
    return task_deps;
}

fn part1(task_deps: &[Vec<u8>]) -> String {
    let mut task_complete = vec![false; task_deps.len()];
    let mut result = String::with_capacity(task_deps.len());

    for _ in 0..task_deps.len() {
        for (task_index, deps) in task_deps
            .iter()
            .enumerate()
            .filter(|(index, _)| !task_complete[*index])
        {
            let all_deps_done = deps.iter().all(|&x| task_complete[x as usize]);
            if all_deps_done {
                result.push((task_index as u8 + 'A' as u8) as char);
                task_complete[task_index] = true;
                break;
            }
        }
    }

    return result;
}

#[derive(Debug, Clone)]
struct TaskStatus {
    assigned: Cell<bool>,
    complete: Cell<bool>,
}

#[derive(Debug, Copy, Clone)]
struct AssignedTask {
    time_remaining: usize,
    task: u8,
}

fn assign_tasks(
    workers: &mut [Cell<Option<AssignedTask>>],
    task_deps: &[Vec<u8>],
    task_status: &[TaskStatus],
) {
    let available_tasks = || {
        task_deps
            .iter()
            .enumerate()
            .filter(|(index, deps)| {
                !task_status[*index].assigned.get()
                    && deps.iter().all(|&x| task_status[x as usize].complete.get())
            })
            .map(|(index, _)| index as u8)
    };

    let available_workers = || workers.iter().filter(|&c| c.get().is_none());

    for (worker, task) in available_workers().zip(available_tasks()) {
        worker.set(Some(AssignedTask {
            time_remaining: 61 + task as usize,
            task: task,
        }));
        task_status[task as usize].assigned.set(true);
    }
}

fn do_work(workers: &mut [Cell<Option<AssignedTask>>], task_status: &[TaskStatus]) -> usize {
    let min_time = workers
        .iter()
        .filter_map(|x| x.get().map(|x| x.time_remaining))
        .min()
        .unwrap_or(0);

    for worker in workers {
        match worker.get() {
            Some(mut t) => {
                t.time_remaining -= min_time;
                if t.time_remaining == 0 {
                    task_status[t.task as usize].complete.set(true);
                    worker.set(None);
                } else {
                    worker.set(Some(t));
                }
            }
            None => {}
        }
    }
    return min_time;
}

fn part2(task_deps: &[Vec<u8>]) -> usize {
    let tasks_status = vec![
        TaskStatus {
            assigned: Cell::from(false),
            complete: Cell::from(false)
        };
        task_deps.len()
    ];

    let mut workers: Vec<Cell<Option<AssignedTask>>> = vec![Cell::new(Option::None); 5];

    let mut total_time = 0;
    loop {
        assign_tasks(workers.as_mut_slice(), task_deps, tasks_status.as_slice());
        let time_elapsed = do_work(workers.as_mut_slice(), tasks_status.as_slice());
        total_time += if time_elapsed == 0 {
            break;
        } else {
            time_elapsed
        };
    }

    return total_time;
}

fn main() {
    let constraints =
        parselib::parse_lines_fn("input.txt", parse_constraints).expect("Could not parse input");
    let task_deps = make_task_deps(constraints.as_slice());
    println!("part 1: {}", part1(task_deps.as_slice()));
    println!("part 2: {}", part2(task_deps.as_slice()));
}
