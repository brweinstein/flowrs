const LEFT: u8 = 1;
const RIGHT: u8 = 2;
const TOP: u8 = 4;
const BOTTOM: u8 = 8;
const DIR_TYPES: [u8; 6] = [
    LEFT | RIGHT,
    TOP | BOTTOM,
    TOP | LEFT,
    TOP | RIGHT,
    BOTTOM | LEFT,
    BOTTOM | RIGHT,
];

use crate::board::{Cell, Colour, Grid, Point};
use std::collections::HashMap;
use varisat::{CnfFormula, ExtendFormula, Solver};

pub fn solve_sat(grid: &Grid) -> Option<Grid> {
    let endpoints = grid.get_endpoints();
    let width = grid.width;
    let height = grid.height;
    let colours: Vec<Colour> = endpoints.keys().cloned().collect();
    let mut var_map = HashMap::new();
    let mut dir_map = HashMap::new();
    let mut next_var = 1;

    // Variable: cell (x, y) is color c
    for y in 0..height {
        for x in 0..width {
            for &colour in &colours {
                var_map.insert((x, y, colour), next_var);
                next_var += 1;
            }
        }
    }
    // Direction type variables for non-endpoints
    for y in 0..height {
        for x in 0..width {
            let point = Point::new(x, y);
            if !matches!(grid.get(point), Cell::Endpoint { .. }) {
                for &dir_type in &DIR_TYPES {
                    dir_map.insert((x, y, dir_type), next_var);
                    next_var += 1;
                }
            }
        }
    }
    let mut formula = CnfFormula::new();

    // Each cell must be exactly one color
    for y in 0..height {
        for x in 0..width {
            let mut clause = Vec::new();
            for &colour in &colours {
                clause.push(var_map[&(x, y, colour)] as isize);
            }
            formula.add_clause(
                &clause
                    .iter()
                    .map(|&v| varisat::Lit::from_dimacs(v))
                    .collect::<Vec<_>>(),
            );
            // At most one color per cell
            for i in 0..colours.len() {
                for j in i + 1..colours.len() {
                    formula.add_clause(&[
                        varisat::Lit::from_dimacs(-(var_map[&(x, y, colours[i])] as isize)),
                        varisat::Lit::from_dimacs(-(var_map[&(x, y, colours[j])] as isize)),
                    ]);
                }
            }
        }
    }

    // Endpoint constraints
    for (&colour, &(p1, p2)) in &endpoints {
        // Endpoint must be its color
        formula.add_clause(&[varisat::Lit::from_dimacs(
            var_map[&(p1.x, p1.y, colour)] as isize,
        )]);
        formula.add_clause(&[varisat::Lit::from_dimacs(
            var_map[&(p2.x, p2.y, colour)] as isize,
        )]);
        // Endpoint must not be any other color
        for &other in &colours {
            if other != colour {
                formula.add_clause(&[varisat::Lit::from_dimacs(
                    -(var_map[&(p1.x, p1.y, other)] as isize),
                )]);
                formula.add_clause(&[varisat::Lit::from_dimacs(
                    -(var_map[&(p2.x, p2.y, other)] as isize),
                )]);
            }
        }
        // Each endpoint must have exactly one neighbor of its color
        for &point in &[p1, p2] {
            let neighbors = point.neighbors(width, height);
            let mut neighbor_vars = Vec::new();
            for n in neighbors {
                neighbor_vars.push(var_map[&(n.x, n.y, colour)] as isize);
            }
            // At least one neighbor
            let clause = neighbor_vars.clone();
            formula.add_clause(
                &clause
                    .iter()
                    .map(|&v| varisat::Lit::from_dimacs(v))
                    .collect::<Vec<_>>(),
            );
            // At most one neighbor
            for i in 0..neighbor_vars.len() {
                for j in i + 1..neighbor_vars.len() {
                    formula.add_clause(&[
                        varisat::Lit::from_dimacs(-neighbor_vars[i]),
                        varisat::Lit::from_dimacs(-neighbor_vars[j]),
                    ]);
                }
            }
        }
    }

    // Direction type constraints for non-endpoints
    for y in 0..height {
        for x in 0..width {
            let point = Point::new(x, y);
            if matches!(grid.get(point), Cell::Endpoint { .. }) {
                continue;
            }
            // Each cell must have exactly one direction type
            let mut dir_vars = Vec::new();
            for &dir_type in &DIR_TYPES {
                if let Some(&v) = dir_map.get(&(x, y, dir_type)) {
                    dir_vars.push(v as isize);
                }
            }
            // At least one direction type
            formula.add_clause(
                &dir_vars
                    .iter()
                    .map(|&v| varisat::Lit::from_dimacs(v))
                    .collect::<Vec<_>>(),
            );
            // At most one direction type
            for i in 0..dir_vars.len() {
                for j in i + 1..dir_vars.len() {
                    formula.add_clause(&[
                        varisat::Lit::from_dimacs(-dir_vars[i]),
                        varisat::Lit::from_dimacs(-dir_vars[j]),
                    ]);
                }
            }
        }
    }

    // Direction type neighbor color constraints and self-touching prevention
    for y in 0..height {
        for x in 0..width {
            let point = Point::new(x, y);
            if matches!(grid.get(point), Cell::Endpoint { .. }) {
                continue;
            }
            for &colour in &colours {
                let cell_var = var_map[&(x, y, colour)] as isize;
                for &dir_type in &DIR_TYPES {
                    if let Some(&dir_var) = dir_map.get(&(x, y, dir_type)) {
                        // Get the two neighbors for this direction type
                        let mut neighbors = Vec::new();
                        match dir_type {
                            d if d == (LEFT | RIGHT) => {
                                neighbors.push((x.wrapping_sub(1), y));
                                neighbors.push((x + 1, y));
                            }
                            d if d == (TOP | BOTTOM) => {
                                neighbors.push((x, y.wrapping_sub(1)));
                                neighbors.push((x, y + 1));
                            }
                            d if d == (TOP | LEFT) => {
                                neighbors.push((x.wrapping_sub(1), y));
                                neighbors.push((x, y.wrapping_sub(1)));
                            }
                            d if d == (TOP | RIGHT) => {
                                neighbors.push((x + 1, y));
                                neighbors.push((x, y.wrapping_sub(1)));
                            }
                            d if d == (BOTTOM | LEFT) => {
                                neighbors.push((x.wrapping_sub(1), y));
                                neighbors.push((x, y + 1));
                            }
                            d if d == (BOTTOM | RIGHT) => {
                                neighbors.push((x + 1, y));
                                neighbors.push((x, y + 1));
                            }
                            _ => {}
                        }
                        // Only add constraints for valid neighbors
                        let valid_neighbors: Vec<_> = neighbors
                            .into_iter()
                            .filter(|&(nx, ny)| nx < width && ny < height)
                            .collect();
                        if valid_neighbors.len() != 2 {
                            continue;
                        }
                        let n1_var =
                            var_map[&(valid_neighbors[0].0, valid_neighbors[0].1, colour)] as isize;
                        let n2_var =
                            var_map[&(valid_neighbors[1].0, valid_neighbors[1].1, colour)] as isize;
                        // yi,t -> (xi,u <-> xj,u) and (xi,u <-> xk,u)
                        // (¬yi,t ∨ ¬xi,u ∨ xj,u)
                        formula.add_clause(&[
                            varisat::Lit::from_dimacs(-(dir_var as isize)),
                            varisat::Lit::from_dimacs(-cell_var),
                            varisat::Lit::from_dimacs(n1_var),
                        ]);
                        // (¬yi,t ∨ xi,u ∨ ¬xj,u)
                        formula.add_clause(&[
                            varisat::Lit::from_dimacs(-(dir_var as isize)),
                            varisat::Lit::from_dimacs(cell_var),
                            varisat::Lit::from_dimacs(-n1_var),
                        ]);
                        // (¬yi,t ∨ ¬xi,u ∨ xk,u)
                        formula.add_clause(&[
                            varisat::Lit::from_dimacs(-(dir_var as isize)),
                            varisat::Lit::from_dimacs(-cell_var),
                            varisat::Lit::from_dimacs(n2_var),
                        ]);
                        // (¬yi,t ∨ xi,u ∨ ¬xk,u)
                        formula.add_clause(&[
                            varisat::Lit::from_dimacs(-(dir_var as isize)),
                            varisat::Lit::from_dimacs(cell_var),
                            varisat::Lit::from_dimacs(-n2_var),
                        ]);
                        // For all other neighbors, prevent self-touching: (¬yi,t ∨ ¬xi,u ∨ ¬xl,u)
                        let all_neighbors = point.neighbors(width, height);
                        for n in all_neighbors {
                            let n_coord = (n.x, n.y);
                            if !valid_neighbors.contains(&n_coord) {
                                let n_var = var_map[&(n.x, n.y, colour)] as isize;
                                formula.add_clause(&[
                                    varisat::Lit::from_dimacs(-(dir_var as isize)),
                                    varisat::Lit::from_dimacs(-cell_var),
                                    varisat::Lit::from_dimacs(-n_var),
                                ]);
                            }
                        }
                    }
                }
            }
        }
    }

    let mut solver = Solver::new();
    solver.add_formula(&formula);
    if solver.solve().unwrap_or(false) {
        if let Some(model) = solver.model() {
            let mut new_grid = grid.clone();
            for y in 0..height {
                for x in 0..width {
                    let mut found = false;
                    for &colour in &colours {
                        let var: isize = var_map[&(x, y, colour)];
                        let idx = (var.abs() - 1) as usize;
                        if idx < model.len() && model[idx].is_positive() {
                            let point = Point::new(x, y);
                            match grid.get(point) {
                                Cell::Endpoint { .. } => {
                                    new_grid.set(point, Cell::Endpoint { colour })
                                }
                                _ => new_grid.set(point, Cell::Path { colour }),
                            }
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        // If no color assigned, puzzle is unsolvable
                        return None;
                    }
                }
            }
            Some(new_grid)
        } else {
            None
        }
    } else {
        None
    }
}
