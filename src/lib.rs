pub mod coord_grid {
    use std::fmt::Debug;
    use std::fs::File;
    use std::io::{BufReader, Lines};
    use strum_macros::EnumIter;

    #[derive(Debug, Clone)]
    pub struct Grid<const GRID_WIDTH: usize, const GRID_HEIGHT: usize, T: Copy + Debug + Default + PartialEq> {
        _grid: [[T; GRID_WIDTH]; GRID_HEIGHT],
    }

    impl<const GRID_WIDTH: usize, const GRID_HEIGHT: usize, T: Copy + Debug + Default + PartialEq> Grid<GRID_WIDTH, GRID_HEIGHT, T> {
        pub fn create() -> Self {
            Self { _grid: [[T::default(); GRID_WIDTH]; GRID_HEIGHT] }
        }

        pub fn shape(&self) -> (usize, usize) {
            (GRID_HEIGHT, GRID_WIDTH)
        }

        pub fn set_all(&mut self, v: T) {
            for i in 0..GRID_HEIGHT {
                for j in 0..GRID_WIDTH {
                    self._grid[i][j] = v;
                }
            }
        }

        pub fn contains_coords(&self, coords: (usize, usize)) -> bool {
            let (i, j) = coords;
            i < GRID_HEIGHT && j < GRID_WIDTH
        }

        pub fn contains_coords_signed(&self, coords: (isize, isize)) -> bool {
            let (i, j) = coords;
            i >= 0 && j >= 0 && self.contains_coords((i.try_into().unwrap(), j.try_into().unwrap()))
        }

        pub fn get(&self, coords: (usize, usize)) -> Result<T, ()> {
            let (i, j) = coords;
            if self.contains_coords(coords) {
                Ok(self._grid[i][j])
            } else {
                Err(())
            }
        }

        pub fn set(&mut self, coords: (usize, usize), v: T) -> Result<(), ()> {
            let (i, j) = coords;
            if self.contains_coords(coords) {
                self._grid[i][j] = v;
                Ok(())
            } else {
                Err(())
            }
        }

        pub fn iter_rows(&self) -> impl Iterator<Item=&[T; GRID_WIDTH]> {
            self._grid.iter()
        }

        pub fn iter(&self) -> impl Iterator<Item=&T> {
            self._grid.iter().flat_map(|row| row.iter())
        }

        pub fn iter_coords(&self) -> impl Iterator<Item=(usize, usize)> {
            (0..GRID_HEIGHT).flat_map(|i| (0..GRID_WIDTH).map(move |j| (i.clone(), j)))
        }

        pub fn iter_values_with_coords(&self) -> impl Iterator<Item=((usize, usize), T)> {
            (0..GRID_HEIGHT).flat_map(move |i| (0..GRID_WIDTH).map(move |j| ((i.clone(), j), self._grid[i][j])))
        }
        
        pub fn move_coords(&self, coords: (usize, usize), direction: Direction) -> Result<(usize, usize), ()> {
            match direction {
                Direction::North => if coords.0 > 0 { Ok((coords.0 - 1, coords.1)) } else { Err(()) },
                Direction::East => if coords.1 < GRID_WIDTH - 1 { Ok((coords.0, coords.1 + 1)) } else { Err(()) },
                Direction::South => if coords.0 < GRID_HEIGHT - 1 { Ok((coords.0 + 1, coords.1)) } else { Err(()) },
                Direction::West => if coords.1 > 0 { Ok((coords.0, coords.1 - 1)) } else { Err(()) },
                Direction::Northeast => self.move_coords(self.move_coords(coords, Direction::North)?, Direction::East),
                Direction::Southeast => self.move_coords(self.move_coords(coords, Direction::South)?, Direction::East),
                Direction::Southwest => self.move_coords(self.move_coords(coords, Direction::South)?, Direction::West),
                Direction::Northwest => self.move_coords(self.move_coords(coords, Direction::North)?, Direction::West),
            }
        }

        pub fn force_move_coords(&self, coords: (usize, usize), direction: Direction) -> (isize, isize) {
            let coords: (isize, isize) = (coords.0.try_into().unwrap(), coords.1.try_into().unwrap());
            match direction {
                Direction::North => (coords.0 - 1, coords.1),
                Direction::East => (coords.0, coords.1 + 1),
                Direction::South => (coords.0 + 1, coords.1),
                Direction::West => (coords.0, coords.1 - 1),
                Direction::Northeast => (coords.0 - 1, coords.1 + 1),
                Direction::Southeast => (coords.0 + 1, coords.1 + 1),
                Direction::Southwest => (coords.0 + 1, coords.1 - 1),
                Direction::Northwest => (coords.0 - 1, coords.1 - 1),
            }
        }

        /// Return a clone of `self`, rotated 90° clockwise `n_rot` times.
         pub fn rotated(&self, n_rot: usize) -> Self {
            match n_rot % 4 {
                0 => self.clone(),
                1 => self.rotated_90(),
                2 => self.rotated_180(),
                3 => self.rotated_270(),
                _ => unreachable!(),
            }
        }

        /// Return a clone of `self`, rotated 90° clockwise.
        pub fn rotated_90(&self) -> Self {
            if ! GRID_WIDTH == GRID_HEIGHT { panic!("cannot rotate a non-square grid by 90°") }
            let mut new_present = Self::create();
            self.iter_values_with_coords().for_each(|((i, j), v)| {
                let i_new = j;
                let j_new = GRID_WIDTH - i - 1;
                new_present.set((i_new, j_new), v).unwrap();
            });
            new_present
        }

        /// Return a clone of `self`, rotated 180° clockwise.
        pub fn rotated_180(&self) -> Self {
            let mut new_present = Grid::create();
            self.iter_values_with_coords().for_each(|((i, j), v)| {
                let i_new = GRID_HEIGHT - i - 1;
                let j_new = GRID_WIDTH - j - 1;
                new_present.set((i_new, j_new), v).unwrap();
            });
            new_present
        }

        /// Return a clone of `self`, rotated 270° clockwise.
        pub fn rotated_270(&self) -> Self {
            if ! GRID_WIDTH == GRID_HEIGHT { panic!("cannot rotate a non-square grid by 270°") }
            let mut new_present = Grid::create();
            self.iter_values_with_coords().for_each(|((i, j), v)| {
                let i_new = GRID_HEIGHT - j - 1;
                let j_new = i;
                new_present.set((i_new, j_new), v).unwrap();
            });
            new_present
        }
        pub fn position(&self, needle: T) -> Option<(usize, usize)> {
            for i in 0..GRID_HEIGHT {
                for j in 0..GRID_WIDTH {
                    if self.get((i, j)).unwrap() == needle {
                        return Some((i, j));
                    }
                }
            }
            None
        }

        pub fn raw(&self) -> &[[T; GRID_WIDTH]; GRID_HEIGHT] {
            &self._grid
        }

        pub fn raw_mut(&mut self) -> &mut [[T; GRID_WIDTH]; GRID_HEIGHT] {
            &mut self._grid
        }
    }

    pub fn file_lines_to_char_grid<const GRID_WIDTH: usize, const GRID_HEIGHT: usize>(lines: Lines<BufReader<File>>) -> Result<Grid<GRID_WIDTH, GRID_HEIGHT, char>, String> {
        let mut grid = Grid::create();
        let mut num_lines = 0;
        let mut line_lengths_valid = true;
        lines.enumerate().for_each(|(i, l)| {
            num_lines += 1;
            let line = l.unwrap();
            if line.len() != GRID_WIDTH { line_lengths_valid = false }
            line.chars().enumerate().for_each(|(j, c)| { let _ = grid.set((i, j), c); });
        });
        if num_lines != GRID_HEIGHT {
            Err(format!("incorrect number of lines: found {num_lines}, expected {GRID_HEIGHT}"))
        } else if ! line_lengths_valid {
            Err(format!("incorrect line length: expected {GRID_WIDTH} characters"))
        } else {
            Ok(grid)
        }
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone, EnumIter, Hash)]
    pub enum Direction {
        North,
        Northeast,
        East,
        Southeast,
        South,
        Southwest,
        West,
        Northwest,
    }
    
    impl Direction {
        pub fn turn_45_anticlockwise(&self) -> Self {
            match self {
                Self::North => Self::Northwest,
                Self::Northeast => Self::North,
                Self::East => Self::Northeast,
                Self::Southeast => Self::East,
                Self::South => Self::Southeast,
                Self::Southwest => Self::South,
                Self::West => Self::Southwest,
                Self::Northwest => Self::West,
            }
        }

        pub fn turn_90_clockwise(&self) -> Self {
            match self {
                Self::North => Self::East,
                Self::Northeast => Self::Southeast,
                Self::East => Self::South,
                Self::Southeast => Self::Southwest,
                Self::South => Self::West,
                Self::Southwest => Self::Northwest,
                Self::West => Self::North,
                Self::Northwest => Self::Northeast,
            }
        }

        pub fn turn_90_anticlockwise(&self) -> Self {
            match self {
                Self::North => Self::West,
                Self::Northeast => Self::Northwest,
                Self::East => Self::North,
                Self::Southeast => Self::Northeast,
                Self::South => Self::East,
                Self::Southwest => Self::Southeast,
                Self::West => Self::South,
                Self::Northwest => Self::Southwest,
            }
        }
        
        pub fn main_directions() -> [Self; 4] {
            [Self::North, Self::East, Self::South, Self::West]
        }

        pub fn turn_anticlockwise(&self, degrees: usize) -> Result<Self, &str> {
            if degrees % 45 != 0 {
                Err("invalid turn angle - must be multiple of 45")
            } else {
                let num_45deg_turns = degrees / 45;
                let mut new_dir = self.clone();
                for _ in 0..num_45deg_turns { new_dir = new_dir.turn_45_anticlockwise(); }
                Ok(new_dir)
            }
        }
    }
}
