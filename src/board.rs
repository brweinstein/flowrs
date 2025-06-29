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
    Endpoint { colour: Colour },
    Path { colour: Colour },
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

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Cell::Empty => ".".to_string(),
            Cell::Endpoint { colour } => format_coloured('O', colour),
            Cell::Path { colour } => format_coloured('o', colour),
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
            cells[p1.y][p1.x] = Cell::Endpoint { colour };
            cells[p2.y][p2.x] = Cell::Endpoint { colour };
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
                if let Cell::Endpoint { colour } = self.get(p) {
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
                    Cell::Path { colour: c } | Cell::Endpoint { colour: c } if c == colour => {
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

        fn border_path_exists(
            grid: &Grid,
            start: Point,
            end: Point,
            colour: Colour,
        ) -> Option<Vec<Point>> {
            use std::collections::{HashMap, VecDeque};

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

                    if !on_border(neighbor, grid.width, grid.height) {
                        continue;
                    }

                    match grid.get(neighbor) {
                        Cell::Empty => {
                            visited.insert(neighbor);
                            came_from.insert(neighbor, current);
                            queue.push_back(neighbor);
                        }
                        Cell::Path { colour: c } | Cell::Endpoint { colour: c } if c == colour => {
                            visited.insert(neighbor);
                            came_from.insert(neighbor, current);
                            queue.push_back(neighbor);
                        }
                        _ => {}
                    }
                }
            }
            None
        }

        let mut updates: Vec<(Point, Cell)> = Vec::new();

        for (&colour, &(start, end)) in endpoints {
            // Only try if both endpoints are on border
            if !(on_border(start, self.width, self.height)
                && on_border(end, self.width, self.height))
            {
                continue;
            }

            if let Some(path) = border_path_exists(self, start, end, colour) {
                for &p in &path {
                    if let Cell::Empty = self.get(p) {
                        updates.push((p, Cell::Path { colour }));
                    }
                }
            }
        }

        for (p, cell) in updates {
            self.set(p, cell);
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
