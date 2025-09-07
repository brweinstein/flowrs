use crate::board::{Cell, Colour, Grid, Point};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

#[derive(Clone, Eq, PartialEq)]
struct State {
    grid: Grid,
    endpoints: HashMap<Colour, (Point, Point)>,
    paths: HashMap<Colour, Vec<Point>>,
    cost: usize,
    estimate: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimate.cmp(&self.estimate)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(grid: &Grid) -> usize {
    grid.cells
        .iter()
        .flatten()
        .filter(|c| matches!(c, Cell::Empty))
        .count()
}

fn dead_end(grid: &Grid) -> bool {
    let width = grid.width;
    let height = grid.height;
    for y in 0..height {
        for x in 0..width {
            let p = Point::new(x, y);
            if matches!(grid.get(p), Cell::Empty) {
                let mut walls = 0;
                for n in p.neighbors(width, height) {
                    match grid.get(n) {
                        Cell::Path { .. } | Cell::Endpoint { .. } => walls += 1,
                        _ => {}
                    }
                }
                if walls >= 3 {
                    return true;
                }
            }
        }
    }
    false
}

fn label_regions(grid: &Grid) -> Vec<Vec<i32>> {
    let width = grid.width;
    let height = grid.height;
    let mut labels = vec![vec![-1; width]; height];
    let mut next_label = 0;
    for y in 0..height {
        for x in 0..width {
            if matches!(grid.cells[y][x], Cell::Empty) && labels[y][x] == -1 {
                let mut queue = VecDeque::new();
                queue.push_back((x, y));
                labels[y][x] = next_label;
                while let Some((cx, cy)) = queue.pop_front() {
                    let p = Point::new(cx, cy);
                    for n in p.neighbors(width, height) {
                        if matches!(grid.get(n), Cell::Empty) && labels[n.y][n.x] == -1 {
                            labels[n.y][n.x] = next_label;
                            queue.push_back((n.x, n.y));
                        }
                    }
                }
                next_label += 1;
            }
        }
    }
    labels
}

fn stranded(
    grid: &Grid,
    endpoints: &HashMap<Colour, (Point, Point)>,
    paths: &HashMap<Colour, Vec<Point>>,
) -> bool {
    let width = grid.width;
    let height = grid.height;
    let labels = label_regions(grid);
    let mut region_touches = vec![HashSet::new(); width * height];
    let mut colour_touches = HashMap::new();
    for (&colour, path) in paths {
        let &last = path.last().unwrap();
        let (_start, end) = endpoints[&colour];
        for p in &[last, end] {
            for n in p.neighbors(width, height) {
                if matches!(grid.get(n), Cell::Empty) {
                    let label = labels[n.y][n.x];
                    if label >= 0 {
                        region_touches[label as usize].insert(colour);
                        colour_touches
                            .entry(colour)
                            .or_insert_with(HashSet::new)
                            .insert(label);
                    }
                }
            }
        }
    }
    // For each non-completed color, must exist a region touching both its current and goal
    for (&colour, path) in paths {
        let &last = path.last().unwrap();
        let (_start, end) = endpoints[&colour];
        if last == end {
            continue;
        }
        let mut found = false;
        for label in 0..region_touches.len() {
            if region_touches[label].contains(&colour)
                && colour_touches
                    .get(&colour)
                    .map_or(false, |s| s.contains(&(label as i32)))
            {
                found = true;
                break;
            }
        }
        if !found {
            return true;
        }
    }
    // For each region, must exist a color whose current and goal touch it
    for label in 0..region_touches.len() {
        if !region_touches[label].is_empty() {
            let mut found = false;
            for (&colour, path) in paths {
                let &last = path.last().unwrap();
                let (_start, end) = endpoints[&colour];
                if last == end {
                    continue;
                }
                if colour_touches
                    .get(&colour)
                    .map_or(false, |s| s.contains(&(label as i32)))
                {
                    found = true;
                    break;
                }
            }
            if !found {
                return true;
            }
        }
    }
    false
}

fn get_forced_moves(
    grid: &Grid,
    endpoints: &HashMap<Colour, (Point, Point)>,
    paths: &HashMap<Colour, Vec<Point>>,
) -> Option<(Colour, Point)> {
    let width = grid.width;
    let height = grid.height;
    for (&colour, path) in paths {
        let &last = path.last().unwrap();
        let (_start, end) = endpoints[&colour];
        if last == end {
            continue;
        }
        let mut moves = Vec::new();
        for n in last.neighbors(width, height) {
            if matches!(grid.get(n), Cell::Empty) {
                moves.push(n);
            }
        }
        if moves.len() == 1 {
            return Some((colour, moves[0]));
        }
    }
    // Forced move: empty cell adjacent to only one flow
    for y in 0..height {
        for x in 0..width {
            let p = Point::new(x, y);
            if matches!(grid.get(p), Cell::Empty) {
                let mut flow_colours = Vec::new();
                for n in p.neighbors(width, height) {
                    for (&colour, path) in paths {
                        let &last = path.last().unwrap();
                        if n == last {
                            flow_colours.push(colour);
                        }
                    }
                }
                if flow_colours.len() == 1 {
                    return Some((flow_colours[0], p));
                }
            }
        }
    }
    None
}

fn get_active_colour(
    grid: &Grid,
    endpoints: &HashMap<Colour, (Point, Point)>,
    paths: &HashMap<Colour, Vec<Point>>,
) -> Option<Colour> {
    let width = grid.width;
    let height = grid.height;
    let mut min_moves = usize::MAX;
    let mut active: Option<Colour> = None;
    for (&colour, path) in paths {
        let &last = path.last().unwrap();
        let (_start, end) = endpoints[&colour];
        if last == end {
            continue;
        }
        let moves = last
            .neighbors(width, height)
            .into_iter()
            .filter(|&n| matches!(grid.get(n), Cell::Empty))
            .count();
        if moves < min_moves {
            min_moves = moves;
            active = Some(colour);
        }
    }
    active
}

pub fn solve_astar(grid: Grid) -> Option<Grid> {
    let endpoints = grid.get_endpoints();
    let mut initial_paths = HashMap::new();
    for (&colour, &(start, _end)) in &endpoints {
        initial_paths.insert(colour, vec![start]);
    }
    let initial_state = State {
        grid: grid.clone(),
        endpoints: endpoints.clone(),
        paths: initial_paths,
        cost: 0,
        estimate: heuristic(&grid),
    };
    let mut open = BinaryHeap::new();
    open.push(initial_state);
    let mut visited = HashSet::new();
    while let Some(mut state) = open.pop() {
        let grid_hash = state.grid.cells.iter().flatten().fold(0u64, |acc, c| {
            acc.wrapping_mul(31).wrapping_add(match c {
                Cell::Empty => 0,
                Cell::Endpoint { colour } => 1 + (*colour as u64),
                Cell::Path { colour } => 100 + (*colour as u64),
            })
        });
        if visited.contains(&grid_hash) {
            continue;
        }
        visited.insert(grid_hash);
        if dead_end(&state.grid) || stranded(&state.grid, &state.endpoints, &state.paths) {
            continue;
        }
        loop {
            if let Some((colour, forced_move)) =
                get_forced_moves(&state.grid, &state.endpoints, &state.paths)
            {
                let mut new_grid = state.grid.clone();
                new_grid.set(forced_move, Cell::Path { colour });
                let mut new_paths = state.paths.clone();
                let mut new_path = new_paths.get(&colour).unwrap().clone();
                new_path.push(forced_move);
                new_paths.insert(colour, new_path);
                state = State {
                    grid: new_grid.clone(),
                    endpoints: state.endpoints.clone(),
                    paths: new_paths,
                    cost: state.cost,
                    estimate: heuristic(&new_grid),
                };
            } else {
                break;
            }
        }
        if state.grid.is_solved(&state.endpoints) {
            return Some(state.grid);
        }
        if let Some(active_colour) = get_active_colour(&state.grid, &state.endpoints, &state.paths)
        {
            let path = state.paths.get(&active_colour).unwrap();
            let &last = path.last().unwrap();
            let (_start, end) = state.endpoints[&active_colour];
            if last == end {
                continue;
            }
            for neighbor in last.neighbors(state.grid.width, state.grid.height) {
                match state.grid.get(neighbor) {
                    Cell::Empty => {
                        let mut new_grid = state.grid.clone();
                        new_grid.set(
                            neighbor,
                            Cell::Path {
                                colour: active_colour,
                            },
                        );
                        let mut new_paths = state.paths.clone();
                        let mut new_path = path.clone();
                        new_path.push(neighbor);
                        new_paths.insert(active_colour, new_path);
                        let cost = state.cost + 1;
                        let estimate = cost + heuristic(&new_grid);
                        open.push(State {
                            grid: new_grid,
                            endpoints: state.endpoints.clone(),
                            paths: new_paths,
                            cost,
                            estimate,
                        });
                    }
                    Cell::Endpoint { colour: c } if c == active_colour => {
                        let new_grid = state.grid.clone();
                        let mut new_paths = state.paths.clone();
                        let mut new_path = path.clone();
                        new_path.push(neighbor);
                        new_paths.insert(active_colour, new_path);
                        let cost = state.cost + 1;
                        let estimate = cost + heuristic(&new_grid);
                        open.push(State {
                            grid: new_grid,
                            endpoints: state.endpoints.clone(),
                            paths: new_paths,
                            cost,
                            estimate,
                        });
                    }
                    _ => {}
                }
            }
        }
    }
    None
}
