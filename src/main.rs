use flowrs::backtracking::{brute_force, SolveResult};
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
        let grid = grid_from_txt(path.clone());

        // Brute force
        let mut brute_grid = grid.clone();
        let (brute_result, brute_dur) = {
            let mut grid = brute_grid.clone();
            let dur = duration(|| {brute_force(&mut grid); });
            (brute_force(&mut brute_grid), dur)
        };
        println!("Brute Force Duration: {:?}", brute_dur);
        match brute_result {
            SolveResult::Solved => println!("Brute force solved the puzzle."),
            SolveResult::Impossible => println!("Brute force failed to solve."),
        }

        println!(" Brute Force Result\n{}", brute_grid);
    }
}