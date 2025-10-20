use crate::board::Grid;
use crate::backtracking::{brute_force, SolveResult};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    PuzzleSelection,
    Solving,
    ViewingSolution,
}

pub struct App {
    pub state: AppState,
    pub selected_puzzle_index: usize,
    pub puzzle_files: Vec<String>,
    pub current_grid: Option<Grid>,
    pub solved_grid: Option<Grid>,
    pub solving_grid: Arc<Mutex<Option<Grid>>>,
    pub solve_duration: Option<Duration>,
    pub solve_result: Option<String>,
    pub should_quit: bool,
    pub steps_count: Arc<Mutex<usize>>,
    pub solve_start_time: Option<Instant>,
    pub current_message: Arc<Mutex<String>>,
    pub solving_thread: Option<thread::JoinHandle<()>>,
}

impl App {
    pub fn new() -> Self {
        let puzzle_files = vec![
            "regular_5x5_01.txt".to_string(),
            "regular_6x6_01.txt".to_string(),
            "regular_7x7_01.txt".to_string(),
        ];

        Self {
            state: AppState::PuzzleSelection,
            selected_puzzle_index: 0,
            puzzle_files,
            current_grid: None,
            solved_grid: None,
            solving_grid: Arc::new(Mutex::new(None)),
            solve_duration: None,
            solve_result: None,
            should_quit: false,
            steps_count: Arc::new(Mutex::new(0)),
            solve_start_time: None,
            current_message: Arc::new(Mutex::new(String::new())),
            solving_thread: None,
        }
    }

    pub fn next_puzzle(&mut self) {
        if self.selected_puzzle_index < self.puzzle_files.len() - 1 {
            self.selected_puzzle_index += 1;
        }
    }

    pub fn previous_puzzle(&mut self) {
        if self.selected_puzzle_index > 0 {
            self.selected_puzzle_index -= 1;
        }
    }

    pub fn load_puzzle(&mut self) {
        use crate::utils::grid_from_txt;
        use std::path::PathBuf;

        let filename = &self.puzzle_files[self.selected_puzzle_index];
        let path = PathBuf::from(format!("puzzles/{}", filename));
        self.current_grid = Some(grid_from_txt(path));
        self.solved_grid = None;
        *self.solving_grid.lock().unwrap() = None;
        self.solve_duration = None;
        self.solve_result = None;
        *self.steps_count.lock().unwrap() = 0;
        self.solve_start_time = None;
    }

    pub fn solve_puzzle(&mut self) {
        if let Some(grid) = &self.current_grid {
            self.state = AppState::Solving;
            *self.solving_grid.lock().unwrap() = Some(grid.clone());
            *self.steps_count.lock().unwrap() = 0;
            self.solve_start_time = Some(Instant::now());
        }
    }

    pub fn step_solve(&mut self) -> bool {
        // This will be called repeatedly during solving
        // Return true if solving is complete
        let mut solving_grid_lock = self.solving_grid.lock().unwrap();
        if let Some(solving_grid) = solving_grid_lock.as_mut() {
            *self.steps_count.lock().unwrap() += 1;
            
            // Perform one step of the backtracking algorithm
            // For now, we'll run the full solver and capture the result
            let mut grid_copy = solving_grid.clone();
            let solve_res = brute_force(&mut grid_copy);
            
            let is_complete = match solve_res {
                SolveResult::Solved => true,
                SolveResult::Impossible => true,
            };

            if is_complete {
                self.solved_grid = Some(grid_copy);
                self.solve_duration = self.solve_start_time.map(|start| start.elapsed());
                self.solve_result = Some(match solve_res {
                    SolveResult::Solved => "Solved!".to_string(),
                    SolveResult::Impossible => "Impossible".to_string(),
                });
                self.state = AppState::ViewingSolution;
                return true;
            }
            
            false
        } else {
            true
        }
    }

    pub fn back_to_selection(&mut self) {
        self.state = AppState::PuzzleSelection;
        self.current_grid = None;
        self.solved_grid = None;
        *self.solving_grid.lock().unwrap() = None;
        self.solve_duration = None;
        self.solve_result = None;
        *self.steps_count.lock().unwrap() = 0;
        self.solve_start_time = None;
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
