use crate::*;
use std::collections::{HashMap, HashSet};

/// Represents the outcome of brute-force solving.
pub enum SolveResult {
    Solved,
    Impossible,
}

/// Finds all possible paths for a given colour from `current` to `end`,
/// traversing only valid cells for that colour.
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

/// Attempts to solve the puzzle by brute-force enumeration of valid paths.
/// Returns `SolveResult::Solved` if a complete fill is found,
/// or `SolveResult::Impossible` if no solution exists.
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

        // Prune if no path exists for this colour
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