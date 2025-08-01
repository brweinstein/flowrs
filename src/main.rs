use flowrs::backtracking::{brute_force, SolveResult};
use flowrs::solver::ai_solver;
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

        // ——— Brute Force ———
        let mut brute_grid = grid.clone();
        let brute_dur = duration(|| brute_force(&mut brute_grid));
        let brute_res = brute_force(&mut brute_grid);
        println!("Brute Force Duration: {:?}", brute_dur);
        match brute_res {
            SolveResult::Solved => println!("Brute force solved the puzzle."),
            SolveResult::Impossible => println!("Brute force failed to solve."),
        }
        println!("--- Brute Force Solution ---\n{}", brute_grid);

        // ——— AI Solver ———
        let mut ai_grid = grid.clone();
        let ai_dur = duration(|| ai_solver(&mut ai_grid));
        let ai_res = ai_solver(&mut ai_grid);
        println!("AI Solver Duration:    {:?}", ai_dur);
        match ai_res {
            SolveResult::Solved => println!("AI solver solved the puzzle."),
            SolveResult::Impossible => println!("AI solver failed to solve."),
        }
        println!("--- AI Solver Solution ---\n{}", ai_grid);

        if brute_grid == ai_grid {
            println!("Both solvers agree on the solution\n");
        } else {
            println!("Solvers produced different solutions\n");
        }

        println!("========================================\n");
    }
}