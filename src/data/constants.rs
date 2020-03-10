pub const TERMWIDTH: usize = 80;

pub enum Direction {
  N, S, E, W
}

impl Direction {
  pub fn from(dir: String) -> Direction {
    match dir.as_str() {
      "n" => Direction::N,
      "s" => Direction::S,
      "e" => Direction::E,
      "w" => Direction::W,
      _ => Direction::N
    }
  }
}

pub const MAP_Y_MIN: usize = 0;
pub const MAP_Y_MAX: usize = 9;
pub const MAP_X_MIN: usize = 0;
pub const MAP_X_MAX: usize = 9;
