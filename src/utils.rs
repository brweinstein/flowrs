use crate::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::time::Instant;

pub fn duration<F>(f: F) -> std::time::Duration
where
    F: FnOnce(),
{
    let start = Instant::now();
    f();
    start.elapsed()
}

pub fn grid_from_txt(path: PathBuf) -> Grid {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut endpoints: HashMap<Colour, Vec<Point>> = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    for (y, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");
        width = line.len().max(width); // In case lines are inconsistent in length

        for (x, ch) in line.chars().enumerate() {
            if ch.is_ascii_alphabetic() {
                let colour = Colour::from_char(ch);
                endpoints.entry(colour).or_default().push(Point { x, y });
            }
        }

        height += 1;
    }

    let endpoints: HashMap<_, _> = endpoints
        .into_iter()
        .map(|(colour, points)| {
            assert!(
                points.len() == 2,
                "Expected 2 endpoints for colour {:?}, got {:?}",
                colour,
                points
            );
            (colour, (points[0], points[1]))
        })
        .collect();

    Grid::new(width, height, &endpoints)
}
