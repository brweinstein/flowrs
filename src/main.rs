use flowrs::astar;
use flowrs::backtracking::{SolveResult, brute_force};
use flowrs::sat;
use flowrs::utils::{duration, grid_from_txt};
use std::path::PathBuf;

fn main() {
    let regular_files = [
        "regular_5x5_01.txt",
        "regular_6x6_01.txt",
        "regular_7x7_01.txt",
        "regular_8x8_01.txt",
        "regular_9x9_01.txt",
    ];

    for filename in &regular_files {
        println!("Benchmarking Puzzle: {}\n", filename);
        let path = PathBuf::from(format!("puzzles/{}", filename));
        let grid = grid_from_txt(path);
        let mut brute_grid = grid.clone();
        let brute_dur = duration(|| {
            brute_force(&mut brute_grid);
        });
        let brute_res = brute_force(&mut brute_grid);
        println!("Brute Force Duration: {:?}", brute_dur);
        match brute_res {
            SolveResult::Solved => println!("Brute force solved the puzzle."),
            SolveResult::Impossible => println!("Brute force failed to solve."),
        }
        println!("{}", brute_grid);
        let astar_dur;
        let astar_grid = {
            let grid_clone = grid.clone();
            astar_dur = duration(|| {
                astar::solve_astar(grid_clone.clone());
            });
            astar::solve_astar(grid.clone())
        };
        println!("A* Solver Duration:    {:?}", astar_dur);
        match &astar_grid {
            Some(g) => println!("{}", g),
            None => println!("A* solver failed to solve."),
        }
        let sat_dur;
        let sat_grid = {
            let grid_clone = grid.clone();
            sat_dur = duration(|| {
                sat::solve_sat(&grid_clone);
            });
            sat::solve_sat(&grid)
        };
        println!("SAT Solver Duration:   {:?}", sat_dur);
        match &sat_grid {
            Some(g) => println!("{}", g),
            None => println!("SAT solver failed to solve."),
        }
        let mut agree = true;
        if let Some(astar_g) = &astar_grid {
            agree &= brute_grid == *astar_g;
        }
        if let Some(sat_g) = &sat_grid {
            agree &= brute_grid == *sat_g;
        }
        if let Some(sat_g) = &sat_grid {
            agree &= brute_grid == *sat_g;
        }
        if agree {
            println!("All solvers agree on the solution\n");
        } else {
            println!("Solvers produced different solutions\n");
        }
    }
}
