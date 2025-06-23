use colored::Colorize;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Colour {
Red, //R
Green, //G
Blue, //B
Yellow, //Y
Magenta, //M
Orange, //O
Cyan, //C
Brown, //m
Purple, //P
White, //W
Gray, //G
Lime, //L
Beige, //b
Navy, //N
Teal,//T
Pink, //p
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
         'm' => Colour::Brown, //Maroon
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
Endpoint(Colour),
Path(Colour),
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
         Cell::Endpoint(colour) => format_coloured('O', colour),
         Cell::Path(colour) => format_coloured('o', colour),
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
   let mut result = Vec::new();
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
   pub fn new(width: usize, height: usize, endpoints: HashMap<Colour, (Point, Point)>) -> Self {
      let mut cells = vec![vec![Cell::Empty; width]; height];

      for (colour, (p1, p2)) in &endpoints {
         cells[p1.y][p1.x] = Cell::Endpoint(*colour);
         cells[p2.y][p2.x] = Cell::Endpoint(*colour);
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

   pub fn add_path(&mut self, point: Point, colour: Colour) {
      let cur = self.get(point);
      match cur {
         Cell::Empty => self.set(point, Cell::Path(colour)),
         Cell::Path(_) => {
               self.set(point, Cell::Path(colour));
         }
         Cell::Endpoint(_) => {
               println!("Cannot add path over endpoint at ({},{})", point.x, point.y);
         }
      }
   }

   pub fn is_solved(&self, endpoints: &HashMap<Colour, (Point, Point)>) -> bool {
      for row in &self.cells {
         for cell in row {
               if let Cell::Empty = cell {
                  return false;
               }
         }
      }

      for (colour, &(start, end)) in endpoints {
         if !self.connected(*colour, start, end) {
               return false;
         }
      }
      true
   }

   fn connected(&self, colour: Colour, start: Point, end: Point) -> bool {
      let mut visited = std::collections::HashSet::new();
      let mut current = start;
      let mut prev = None;

      loop {
         if current == end {
               return true;
         }

         visited.insert(current);

         let next: Vec<Point> = current
               .neighbors(self.width, self.height)
               .into_iter()
               .filter(|&p| Some(p) != prev) // Don't go backward
               .filter(
                  |&p| matches!(self.get(p), Cell::Path(c) | Cell::Endpoint(c) if c == colour),
               )
               .collect();

         if next.len() != 1 {
               return false; //Dead end
         }

         prev = Some(current);
         current = next[0];
      }
   }
   pub fn find_endpoints(self: Self) -> HashMap<Colour, (Point, Point)> {
      let mut endpoints: HashMap<Colour, Vec<Point>> = HashMap::new();

      for (y, row) in self.cells.iter().enumerate() {
         for (x, cell) in row.iter().enumerate() {
               if let Cell::Endpoint(colour) = cell {
                  endpoints.entry(*colour).or_default().push(Point { x, y });
               }
         }
      }

      endpoints
         .into_iter()
         .map(|(colour, points)| {
               assert!(
                  points.len() == 2,
                  "Expected 2 endpoints for colour {:?}, got {}",
                  colour,
                  points.len()
               );
               (colour, (points[0], points[1]))
         })
         .collect()
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
