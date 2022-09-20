use crate::random::random_range;
use std::collections::HashSet;
use std::fmt::Display;

pub type Position = (usize, usize);

pub enum OpenResult {
  Mine,
  NoMine(u8),
}

#[derive(Debug)]
pub struct Minesweeper {
  pub width: usize,
  pub height: usize,
  pub open_fields: HashSet<Position>,
  pub mines: HashSet<Position>,
  pub flagged_fields: HashSet<Position>,
}

impl Display for Minesweeper {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    for x in 0..self.width {
      for y in 0..self.height {
        let pos = (x, y);
        if self.flagged_fields.contains(&pos) {
          f.write_str("🚩 ")?;
        // } else if !self.open_fields.contains(&pos) {
        //     f.write_str("🟪 ")?;
        //     // write!(f, "-({}, {})-", x, y)?;
        } else if self.mines.contains(&pos) {
          f.write_str("💣 ")?;
        } else {
          write!(f, " {} ", self.mine_count((x, y)))?;
        }
      }
      f.write_str("\n")?;
    }
    Ok(())
  }
}

impl Minesweeper {
  pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
    Minesweeper {
      width: width,
      height: height,
      open_fields: HashSet::new(),
      mines: {
        let mut mines = HashSet::new();
        while mines.len() < mine_count {
          mines.insert((random_range(0, width), random_range(0, height)));
        }
        mines
      },
      flagged_fields: HashSet::new(),
    }
  }

  pub fn neighbours(&self, (x, y): Position) -> impl Iterator<Item = Position> {
    let width = self.width;
    let height = self.height;

    (x.max(1) - 1..(x + 2).min(width))
      .flat_map(move |i| (y.max(1) - 1..(y + 2).min(height)).map(move |j| (i, j)))
      .filter(move |&p| p != (x, y))
      .into_iter()
  }

  pub fn mine_count(&self, (x, y): Position) -> usize {
    self
      .neighbours((x, y))
      .filter(|p| self.mines.contains(p))
      .count()
  }

  // Open a position
  pub fn open(&mut self, position: Position) -> Option<OpenResult> {
    if self.flagged_fields.contains(&position) {
      return None;
    }

    self.open_fields.insert(position);
    let is_mine = self.mines.contains(&position);
    if is_mine {
      Some(OpenResult::Mine)
    } else {
      Some(OpenResult::NoMine(0))
    }
  }

  pub fn toggle_flag(&mut self, position: Position) {
    if self.open_fields.contains(&position) {
      return;
    }
    if self.flagged_fields.contains(&position) {
      self.flagged_fields.remove(&position);
    } else {
      self.flagged_fields.insert(position);
    }
  }
}
