use flowrs::*;
use std::collections::HashMap;

#[test]
fn test_manual_solution() {
    let mut endpoints = HashMap::new();
    endpoints.insert(Colour::Red, (Point::new(0, 0), Point::new(1, 4)));
    endpoints.insert(Colour::Green, (Point::new(2, 0), Point::new(1, 3)));
    endpoints.insert(Colour::Blue, (Point::new(2, 1), Point::new(2, 4)));
    endpoints.insert(Colour::Yellow, (Point::new(4, 0), Point::new(3, 3)));
    endpoints.insert(Colour::Magenta, (Point::new(3, 4), Point::new(4, 1)));

    let mut grid = Grid::new(5, 5, endpoints.clone());

    let red = [
        Point::new(0, 1),
        Point::new(0, 2),
        Point::new(0, 3),
        Point::new(0, 4),
    ];
    let green = [Point::new(1, 0), Point::new(1, 1), Point::new(1, 2)];
    let blue = [Point::new(2, 2), Point::new(2, 3)];
    let yellow = [Point::new(3, 0), Point::new(3, 1), Point::new(3, 2)];
    let orange = [Point::new(4, 2), Point::new(4, 3), Point::new(4, 4)];

    for point in red {
        grid.add_path(point, Colour::Red);
    }
    for point in green {
        grid.add_path(point, Colour::Green);
    }
    for point in blue {
        grid.add_path(point, Colour::Blue);
    }
    for point in yellow {
        grid.add_path(point, Colour::Yellow);
    }
    for point in orange {
        grid.add_path(point, Colour::Magenta);
    }

    assert!(
        grid.is_solved(&endpoints),
        "The manually filled puzzle should be solved."
    );
}