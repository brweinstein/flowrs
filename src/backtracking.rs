use crate::board::{Cell, Colour, Grid, Point};
use std::collections::{HashMap, HashSet};

pub enum SolveResult {
    Solved,
    Impossible,
}

pub type StepCallback = Box<dyn FnMut(&Grid, usize, &str) -> bool + Send>;

fn find_paths(
    grid: &Grid,
    current: Point,
    end: Point,
    colour: Colour,
    visited: &mut HashSet<Point>,
    path: &mut Vec<Point>,
) -> Vec<Vec<Point>> {
    if current == end {
        return vec![path.clone()];
    }

    let mut results = vec![];
    visited.insert(current);

    for neighbour in current.neighbors(grid.width, grid.height) {
        if visited.contains(&neighbour) {
            continue;
        }

        match grid.get(neighbour) {
            Cell::Empty => {
                path.push(neighbour);
                let subpaths = find_paths(grid, neighbour, end, colour, visited, path);
                results.extend(subpaths);
                path.pop();
            }
            Cell::Path { colour: c } if c == colour => {
                path.push(neighbour);
                let subpaths = find_paths(grid, neighbour, end, colour, visited, path);
                results.extend(subpaths);
                path.pop();
            }
            Cell::Endpoint { colour: c } if c == colour => {
                path.push(neighbour);
                let subpaths = find_paths(grid, neighbour, end, colour, visited, path);
                results.extend(subpaths);
                path.pop();
            }
            _ => {}
        }
    }

    visited.remove(&current);
    results
}

pub fn brute_force(grid: &mut Grid) -> SolveResult {
    let endpoints = grid.get_endpoints();
    grid.fill_guaranteed(&endpoints);

    let pairs: Vec<(Colour, Point, Point)> =
        endpoints.iter().map(|(&c, &(s, e))| (c, s, e)).collect();

    fn backtrack(
        grid: &mut Grid,
        pairs: &[(Colour, Point, Point)],
        index: usize,
        endpoints: &HashMap<Colour, (Point, Point)>,
    ) -> bool {
        if index == pairs.len() {
            return grid.is_solved(endpoints);
        }

        let (colour, start, end) = pairs[index];

        if grid.connected(colour, start, end) {
            return backtrack(grid, pairs, index + 1, endpoints);
        }

        let mut visited = HashSet::new();
        let mut path = vec![];

        let all_paths = find_paths(grid, start, end, colour, &mut visited, &mut path);

        if all_paths.is_empty() {
            return false;
        }

        for path in all_paths.iter() {
            for &p in path {
                if matches!(grid.get(p), Cell::Empty) {
                    grid.set(p, Cell::Path { colour });
                }
            }

            grid.fill_guaranteed(endpoints);

            if backtrack(grid, pairs, index + 1, endpoints) {
                return true;
            }

            for &p in path {
                if let Cell::Path { colour: c } = grid.get(p) {
                    if c == colour {
                        grid.set(p, Cell::Empty);
                    }
                }
            }
        }

        false
    }

    if backtrack(grid, &pairs, 0, &endpoints) {
        SolveResult::Solved
    } else {
        SolveResult::Impossible
    }
}

pub fn brute_force_with_callback(grid: &mut Grid, mut callback: StepCallback) -> SolveResult {
    let endpoints = grid.get_endpoints();
    grid.fill_guaranteed(&endpoints);

    let pairs: Vec<(Colour, Point, Point)> =
        endpoints.iter().map(|(&c, &(s, e))| (c, s, e)).collect();

    let mut step_count = 0;

    fn backtrack<F>(
        grid: &mut Grid,
        pairs: &[(Colour, Point, Point)],
        index: usize,
        endpoints: &HashMap<Colour, (Point, Point)>,
        callback: &mut F,
        step_count: &mut usize,
    ) -> bool
    where
        F: FnMut(&Grid, usize, &str) -> bool,
    {
        if index == pairs.len() {
            return grid.is_solved(endpoints);
        }

        let (colour, start, end) = pairs[index];

        if grid.connected(colour, start, end) {
            return backtrack(grid, pairs, index + 1, endpoints, callback, step_count);
        }

        let mut visited = HashSet::new();
        let mut path = vec![];

        let all_paths = find_paths(grid, start, end, colour, &mut visited, &mut path);

        if all_paths.is_empty() {
            return false;
        }

        for (path_idx, path) in all_paths.iter().enumerate() {
            *step_count += 1;
            
            for &p in path {
                if matches!(grid.get(p), Cell::Empty) {
                    grid.set(p, Cell::Path { colour });
                }
            }

            grid.fill_guaranteed(endpoints);

            // Call the callback to visualize this step
            let msg = format!("Trying path {} for colour {:?}", path_idx + 1, colour);
            let should_continue = callback(grid, *step_count, &msg);
            if !should_continue {
                return false; // User cancelled
            }

            if backtrack(grid, pairs, index + 1, endpoints, callback, step_count) {
                return true;
            }

            // Backtrack
            *step_count += 1;
            for &p in path {
                if let Cell::Path { colour: c } = grid.get(p) {
                    if c == colour {
                        grid.set(p, Cell::Empty);
                    }
                }
            }
            
            let backtrack_msg = format!("Backtracking from colour {:?}", colour);
            let should_continue = callback(grid, *step_count, &backtrack_msg);
            if !should_continue {
                return false;
            }
        }

        false
    }

    let result = backtrack(grid, &pairs, 0, &endpoints, &mut callback, &mut step_count);
    
    if result {
        SolveResult::Solved
    } else {
        SolveResult::Impossible
    }
}
