use colored::Colorize;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Colour {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Orange,
    Cyan,
    Brown,
    Purple,
    White,
    Gray,
    Lime,
    Beige,
    Navy,
    Teal,
    Pink,
}

impl Colour {
    pub fn from_char(c: char) -> Self {
        match c {
            'R' => Colour::Red,
            'B' => Colour::Blue,
            'G' => Colour::Green,
            'M' => Colour::Magenta,
            'Y' => Colour::Yellow,
            'O' => Colour::Orange,
            'C' => Colour::Cyan,
            'm' => Colour::Brown,
            'P' => Colour::Purple,
            'W' => Colour::White,
            'g' => Colour::Gray,
            'L' => Colour::Lime,
            'b' => Colour::Beige,
            'N' => Colour::Navy,
            'T' => Colour::Teal,
            'p' => Colour::Pink,
            _ => panic!("Invalid char"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cell {
    Empty,
    Endpoint { colour: Colour, solved: bool },
    Path { colour: Colour, solved: bool },
}

fn format_coloured(c: char, colour: &Colour) -> String {
    match colour {
        Colour::Red => c.to_string().red().to_string(),
        Colour::Green => c.to_string().green().to_string(),
        Colour::Blue => c.to_string().blue().to_string(),
        Colour::Yellow => c.to_string().yellow().to_string(),
        Colour::Magenta => c.to_string().magenta().to_string(),
        Colour::Orange => c.to_string().truecolor(255, 165, 0).to_string(),
        Colour::Cyan => c.to_string().cyan().to_string(),
        Colour::Brown => c.to_string().truecolor(139, 69, 19).to_string(),
        Colour::Purple => c.to_string().truecolor(128, 0, 128).to_string(),
        Colour::White => c.to_string().white().to_string(),
        Colour::Gray => c.to_string().bright_black().to_string(),
        Colour::Lime => c.to_string().truecolor(0, 255, 0).to_string(),
        Colour::Beige => c.to_string().truecolor(245, 245, 220).to_string(),
        Colour::Navy => c.to_string().truecolor(0, 0, 128).to_string(),
        Colour::Teal => c.to_string().truecolor(0, 128, 128).to_string(),
        Colour::Pink => c.to_string().truecolor(255, 192, 203).to_string(),
    }
}

impl Cell {
    pub fn is_solved(&self) -> bool {
        match self {
            Cell::Empty => false,
            Cell::Endpoint { solved, .. } => *solved,
            Cell::Path { solved, .. } => *solved,
        }
    }

    pub fn colour(&self) -> Option<Colour> {
        match self {
            Cell::Empty => None,
            Cell::Endpoint { colour, .. } => Some(*colour),
            Cell::Path { colour, .. } => Some(*colour),
        }
    }

    pub fn mark_solved(&self) -> Self {
        match self {
            Cell::Empty => *self,
            Cell::Endpoint { colour, .. } => Cell::Endpoint { colour: *colour, solved: true },
            Cell::Path { colour, .. } => Cell::Path { colour: *colour, solved: true },
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Cell::Empty => ".".to_string(),
            Cell::Endpoint { colour, .. } => format_coloured('O', colour),
            Cell::Path { colour, .. } => format_coloured('o', colour),
        };
        write!(f, "{}", ch)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn neighbors(&self, width: usize, height: usize) -> Vec<Point> {
        let mut result = Vec::with_capacity(4);
        if self.x > 0 {
            result.push(Point::new(self.x - 1, self.y));
        }
        if self.x + 1 < width {
            result.push(Point::new(self.x + 1, self.y));
        }
        if self.y > 0 {
            result.push(Point::new(self.x, self.y - 1));
        }
        if self.y + 1 < height {
            result.push(Point::new(self.x, self.y + 1));
        }
        result
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(width: usize, height: usize, endpoints: &HashMap<Colour, (Point, Point)>) -> Self {
        let mut cells = vec![vec![Cell::Empty; width]; height];

        for (&colour, &(p1, p2)) in endpoints {
            cells[p1.y][p1.x] = Cell::Endpoint { colour, solved: false };
            cells[p2.y][p2.x] = Cell::Endpoint { colour, solved: false };
        }

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn get(&self, point: Point) -> Cell {
        self.cells[point.y][point.x]
    }

    pub fn set(&mut self, point: Point, cell: Cell) {
        self.cells[point.y][point.x] = cell;
    }

    pub fn get_endpoints(&self) -> HashMap<Colour, (Point, Point)> {
        let mut endpoints: HashMap<Colour, Vec<Point>> = HashMap::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point::new(x, y);
                if let Cell::Endpoint { colour, .. } = self.get(p) {
                    endpoints.entry(colour).or_default().push(p);
                }
            }
        }

        endpoints
            .into_iter()
            .map(|(colour, points)| {
                assert!(
                    points.len() == 2,
                    "Each colour must have exactly 2 endpoints"
                );
                (colour, (points[0], points[1]))
            })
            .collect()
    }

    pub fn is_solved(&self, endpoints: &HashMap<Colour, (Point, Point)>) -> bool {
        for row in &self.cells {
            for cell in row {
                if let Cell::Empty = cell {
                    return false;
                }
            }
        }

        for (&colour, &(start, end)) in endpoints {
            if !self.connected(colour, start, end) {
                return false;
            }
        }

        true
    }

    pub fn connected(&self, colour: Colour, start: Point, end: Point) -> bool {
        use std::collections::VecDeque;

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            if current == end {
                return true;
            }

            for neighbor in current.neighbors(self.width, self.height) {
                if visited.contains(&neighbor) {
                    continue;
                }
                match self.get(neighbor) {
                    Cell::Path { colour: c, .. } | Cell::Endpoint { colour: c, .. } if c == colour => {
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                    _ => {}
                }
            }
        }

        false
    }

    pub fn fill_guaranteed(&mut self, endpoints: &HashMap<Colour, (Point, Point)>) {
        fn on_border(p: Point, width: usize, height: usize) -> bool {
            p.x == 0 || p.x == width - 1 || p.y == 0 || p.y == height - 1
        }

        fn is_adjacent_to_solved(grid: &Grid, point: Point) -> bool {
            for neighbor in point.neighbors(grid.width, grid.height) {
                if let Cell::Path { solved: true, .. } | Cell::Endpoint { solved: true, .. } = grid.get(neighbor) {
                    return true;
                }
            }
            false
        }

        fn find_all_paths(
            grid: &Grid,
            current: Point,
            end: Point,
            colour: Colour,
            visited: &mut HashSet<Point>,
            path: &mut Vec<Point>,
        ) -> Vec<Vec<Point>> {
            if current == end {
                return vec![path.clone()];
            }

            let mut results = vec![];
            visited.insert(current);

            for neighbor in current.neighbors(grid.width, grid.height) {
                if visited.contains(&neighbor) {
                    continue;
                }

                match grid.get(neighbor) {
                    Cell::Empty => {
                        path.push(neighbor);
                        let subpaths = find_all_paths(grid, neighbor, end, colour, visited, path);
                        results.extend(subpaths);
                        path.pop();
                    }
                    Cell::Path { colour: c, .. } | Cell::Endpoint { colour: c, .. } if c == colour => {
                        path.push(neighbor);
                        let subpaths = find_all_paths(grid, neighbor, end, colour, visited, path);
                        results.extend(subpaths);
                        path.pop();
                    }
                    _ => {}
                }
            }

            visited.remove(&current);
            results
        }

        fn guaranteed_path_exists(
            grid: &Grid,
            start: Point,
            end: Point,
            colour: Colour,
        ) -> Option<Vec<Point>> {
            use std::collections::{HashMap, VecDeque};

            // First try the restricted approach (border + adjacent to solved)
            let mut queue = VecDeque::new();
            let mut came_from = HashMap::new();
            let mut visited = HashSet::new();

            queue.push_back(start);
            visited.insert(start);

            while let Some(current) = queue.pop_front() {
                if current == end {
                    // reconstruct path
                    let mut path = vec![current];
                    let mut cur = current;
                    while let Some(&prev) = came_from.get(&cur) {
                        path.push(prev);
                        cur = prev;
                    }
                    path.reverse();
                    return Some(path);
                }

                for neighbor in current.neighbors(grid.width, grid.height) {
                    if visited.contains(&neighbor) {
                        continue;
                    }

                    // Can traverse if on border OR adjacent to solved cells
                    let can_traverse = on_border(neighbor, grid.width, grid.height) 
                        || is_adjacent_to_solved(grid, neighbor);

                    if !can_traverse {
                        continue;
                    }

                    match grid.get(neighbor) {
                        Cell::Empty => {
                            visited.insert(neighbor);
                            came_from.insert(neighbor, current);
                            queue.push_back(neighbor);
                        }
                        Cell::Path { colour: c, .. } | Cell::Endpoint { colour: c, .. } if c == colour => {
                            visited.insert(neighbor);
                            came_from.insert(neighbor, current);
                            queue.push_back(neighbor);
                        }
                        _ => {}
                    }
                }
            }

            // If restricted approach failed, try finding all possible paths
            // If there's exactly one path, it's guaranteed
            let mut all_visited = HashSet::new();
            let mut path = vec![start];
            let all_paths = find_all_paths(grid, start, end, colour, &mut all_visited, &mut path);
            
            if all_paths.len() == 1 {
                return Some(all_paths.into_iter().next().unwrap());
            }

            None
        }

        loop {
            let mut updates: Vec<(Point, Cell)> = Vec::new();
            let mut solved_colours: Vec<Colour> = Vec::new();

            for (&colour, &(start, end)) in endpoints {
                if self.connected(colour, start, end) {
                    continue;
                }

                let start_valid = on_border(start, self.width, self.height) 
                    || is_adjacent_to_solved(self, start);
                let end_valid = on_border(end, self.width, self.height) 
                    || is_adjacent_to_solved(self, end);

                if !(start_valid && end_valid) {
                    continue;
                }

                if let Some(path) = guaranteed_path_exists(self, start, end, colour) {
                    let mut has_updates = false;
                    for &p in &path {
                        if let Cell::Empty = self.get(p) {
                            updates.push((p, Cell::Path { colour, solved: true }));
                            has_updates = true;
                        }
                    }
                    if has_updates {
                        solved_colours.push(colour);
                    }
                }
            }

            // If no updates found, we're done
            if updates.is_empty() {
                break;
            }

            // Apply updates
            for (p, cell) in updates {
                self.set(p, cell);
            }

            // Mark all cells of the solved colours as solved
            for colour in solved_colours {
                self.mark_solved(colour);
            }
        }
    }

    pub fn mark_solved(&mut self, colour: Colour) {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point::new(x, y);
                match self.get(point) {
                    Cell::Endpoint { colour: c, .. } if c == colour => {
                        self.set(point, Cell::Endpoint { colour, solved: true });
                    }
                    Cell::Path { colour: c, .. } if c == colour => {
                        self.set(point, Cell::Path { colour, solved: true });
                    }
                    _ => {}
                }
            }
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{} ", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
