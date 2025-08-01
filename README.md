# FlowRS – Solver for Flow Free Puzzles
A Rust-based solver for puzzles from the Flow Free mobile game.

Inspired by mzucker’s flow-solver, this project aims to build efficient, modular solvers for Flow puzzles using Rust. It also includes early-stage support for reinforcement learning agents written in Python (see flowai/).

## Features
- Fully functional brute-force backtracking solver in Rust (backtracking.rs)

- Core puzzle logic and data structures (board.rs)

- File-based puzzle loading and runtime timing tools (utils.rs)

- CLI entry point and modular architecture (main.rs, lib.rs)

- Foundation for AI-based solvers using Python + Torch (flowai/)

## Example Outputs
![5x5 and 6x6](/media/5and6.png)  
![7x7 and 8x8](/media/7and8.png)
The 7x7 grid is slow to solve; 8x8 is significantly longer and may stall without further optimizations.
The 9x9 grid I have yet to solve with the brute force method.

# Project Structure
```text
flowrs/
├── Cargo.toml
├── media/                # Solution images
├── puzzles/              # Puzzle .txt files (5x5 to 14x14)
├── src/
│   ├── backtracking.rs   # Brute-force solver
│   ├── board.rs          # Grid, Point, Cell, Colour, etc.
│   ├── lib.rs            # Project module entry
│   ├── main.rs           # CLI runner
│   ├── puzzle_ai.rs      # Placeholder for AI integration
│   └── utils.rs          # Puzzle file loading, timing utilities
└── flowai/               # Python reinforcement learning module
    ├── flow_env.py
    ├── solve.py
    ├── export_torchscript.py
    ├── visualize.py
    └── models/
```

## Modules Overview

*board.rs*

Defines the core types used in puzzles:
- Point, Colour, Cell, and Grid

- Grid display logic uses 'O' for endpoints and 'o' for paths (rather than letters per colour)

- Supports construction from a HashMap<Colour, [Point; 2]> or a .txt file

- Includes methods like is_solved, path_completed, and formatted output

*backtracking.rs*

Implements the brute-force recursive solver:
- find_paths: recursively finds all valid paths between a pair of endpoints

- brute_force: top-level solver that connects all colour pairs

- backtrack: core recursive logic, with path setting and undoing
The solver is correct but extremely slow for larger grids due to the exponential path combinations.

*utils.rs*
- Loads puzzles from the puzzles/ directory

- Extracts endpoints and builds a Grid

- Provides basic timing utilities for benchmarking solver runtime

*lib.rs*
Central module file that organizes all submodules.

## AI Solver (WIP)

The flowai/ directory contains the skeleton of a reinforcement learning approach for solving puzzles. It includes:
- A custom FlowEnv class (OpenAI Gym-style)

- Python scripts for solving and exporting models

- Pretrained PPO models (for Sudoku, as placeholders)

- TODO: Train models on actual Flow puzzles and integrate into flowrs

## Sample Puzzles
Over 30 .txt files of varying difficulty (from 5x5 to 14x14) are included under the puzzles/ directory. For example:
```text
puzzles/regular_5x5_01.txt
puzzles/extreme_12x12_30.txt
puzzles/jumbo_14x14_19.txt
```
Each file contains coordinates for endpoints of different colours.

## Roadmap

Brute-force solver

Grid/Cell logic with 16-colour support

Smarter heuristics (dead-end pruning, guaranteed paths)

Reinforcement learning agent (Python + Torch)

Web-based puzzle visualizer, WASM/CLI tool release

## Author
bw@bw:~/documents/proj/flowrs
Created by Ben — Honours Math @ University of Waterloo