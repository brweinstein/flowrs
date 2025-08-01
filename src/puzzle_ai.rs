use crate::board::Grid;
use crate::backtracking::SolveResult;
use lazy_static::lazy_static;
use tch::{CModule, Device, Kind, Tensor};

lazy_static! {
    static ref MODEL: CModule =
        CModule::load("flowai/models/sudoku_ts.pt").expect("could not load TorchScript model");
}

pub fn ai_solver(grid: &mut Grid) -> SolveResult {
    // Flatten grid values (0..=9) into f32
    let input_data: Vec<f32> = grid
        .cells
        .iter()
        .map(|c| c.value as f32)
        .collect();

    // Build a [1,1,N,N] tensor
    let size = grid.size as i64;
    let tensor = Tensor::of_slice(&input_data)
        .view([1, 1, size, size])
        .to_device(Device::Cpu);

    // Forward pass
    let output = MODEL
        .forward_ts(&[tensor])
        .expect("model forward failed");
    // output shape: [1, N*N, max_val], pick argmax per cell
    let preds = output
        .softmax(-1, Kind::Float)
        .argmax(-1, false)
        .view([-1])
        .into::<Vec<i64>>();

    // Fill grid
    for (i, &p) in preds.iter().enumerate() {
        let row = i / grid.size;
        let col = i % grid.size;
        grid.set(row, col, (p as u8) + 1);
    }

    SolveResult::Solved
}