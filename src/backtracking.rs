use crate::*;
use std::collections::{HashMap, HashSet};

/// Find all valid paths from `current` to `end` (brute force)
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
            Cell::Endpoint(c) if c == colour => {
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

/// Brute force solver
pub fn brute_force(grid: &mut Grid) -> bool {
    let endpoints: HashMap<Colour, (Point, Point)> = grid.clone().find_endpoints();
    let pairs: Vec<(Colour, Point, Point)> =
        endpoints.iter().map(|(&c, &(s, e))| (c, s, e)).collect();

    fn backtrack(grid: &mut Grid, pairs: &[(Colour, Point, Point)], index: usize) -> bool {
        if index == pairs.len() {
            return grid.is_solved(&pairs.iter().map(|(c, s, e)| (*c, (*s, *e))).collect());
        }

        let (colour, start, end) = pairs[index];
        let mut visited = HashSet::new();
        let mut path = vec![];

        let all_paths = find_paths(grid, start, end, colour, &mut visited, &mut path);

        for path in all_paths {
            for &p in &path {
                if grid.get(p) == Cell::Empty {
                    grid.set(p, Cell::Path(colour));
                }
            }

            if backtrack(grid, pairs, index + 1) {
                return true;
            }

            for &p in &path {
                if grid.get(p) == Cell::Path(colour) {
                    grid.set(p, Cell::Empty);
                }
            }
        }

        false
    }

    backtrack(grid, &pairs, 0)
}
