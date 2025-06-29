use flowrs::backtracking::brute_force;
use flowrs::utils::{duration, grid_from_txt};
use std::path::PathBuf;

fn main() {
    let mut a = grid_from_txt(PathBuf::from("puzzles/regular_8x8_01.txt"));

    println!("{}", a);

    let dur = duration(|| {
        brute_force(&mut a);
    });
    println!("Solver: {:.2?}", dur);

    println!("{}", a)
}
