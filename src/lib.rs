pub mod board;
pub mod backtracking;
pub mod utils;
pub mod puzzle_ai;
pub mod solver;

use crate::puzzle_ai::PuzzleAI;
use anyhow::Result;
use std::process::{Command, Stdio};

/// Spawns the Python RL agent, feeds it the puzzle JSON on stdin,
/// and reads back a JSON list of cell‐indices as the “AI moves”.
pub fn solve_with_ai(grid: &crate::board::Grid) -> Result<Vec<usize>> {
    // Convert our Grid into the minimal PuzzleAI struct
    let pzl: PuzzleAI = grid.into();

    // Launch the Python script
    let mut cmd = Command::new("python3")
        .arg("flow_ai/solve.py")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // Send puzzle JSON
    serde_json::to_writer(cmd.stdin.as_mut().unwrap(), &pzl)?;

    // Wait & parse
    let output = cmd.wait_with_output()?;
    let moves: Vec<usize> = serde_json::from_slice(&output.stdout)?;
    Ok(moves)
}