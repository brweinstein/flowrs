use flowrs::backtracking::brute_force;
use flowrs::utils::{duration, grid_from_txt};
use flowrs::board::{Grid};
use std::path::PathBuf;

fn main() {
    let grids: Vec<Grid> = vec!(
        grid_from_txt(PathBuf::from("puzzles/regular_5x5_01.txt")),
        grid_from_txt(PathBuf::from("puzzles/regular_6x6_01.txt")),
        grid_from_txt(PathBuf::from("puzzles/regular_7x7_01.txt"))
    );

    for mut grid in grids {
        println!("{}", grid);

        let dur = duration(|| {
            brute_force(&mut grid);
        });
        println!("Solver: {:.2?}", dur);

        println!("{}", grid)
    }
}
