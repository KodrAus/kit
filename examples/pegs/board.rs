use crate::constants::BOARD_NUM_SPACES;
use crate::constants::BOARD_SIZE;
use crate::types::Coords;
use crate::types::PegType;

type Slot = Option<usize>;

fn board_i(pos: Coords) -> usize {
  let x = pos.x;
  let y = pos.y;
  match (x, y) {
    (0, 0) => 0,
    (0, 1) => 1,
    (1, 1) => 2,
    (0, 2) => 3,
    (1, 2) => 4,
    (2, 2) => 5,
    (0, 3) => 6,
    (1, 3) => 7,
    (2, 3) => 8,
    (3, 3) => 9,
    (0, 4) => 10,
    (1, 4) => 11,
    (2, 4) => 12,
    (3, 4) => 13,
    (4, 4) => 14,
    (x, y) => panic!("not a valid board position ({}, {})", x, y),
  }
}

/// Represents a triangular game board. The first row is a single space in length
/// and each subsequen row is one unit longer until the final row which will
/// have a number of spaces equal to the `size` of the board.
pub struct Board {
  e: [Slot; BOARD_NUM_SPACES],
  count: usize,
}

impl Default for Board {
  fn default() -> Self {
    Self {
      e: [None; BOARD_NUM_SPACES],
      count: 0,
    }
  }
}

impl Board {
  pub fn wipe(&mut self) {
    for i in 0..BOARD_NUM_SPACES {
      self.e[i] = None;
    }
    self.count = 0;
  }

  pub fn set(&mut self, pos: Coords, item: Slot) {
    let i = board_i(pos);

    match (self.e[i], item) {
      (None, Some(_)) => {
        self.count += 1;
      }
      (Some(_), None) => {
        self.count -= 1;
      }
      _ => {}
    }

    self.e[i] = item;
  }

  pub fn get(&self, pos: Coords) -> Option<usize> {
    let i = board_i(pos);
    self.e[i]
  }

  pub fn count(&self) -> usize {
    self.count
  }

  pub fn iterator(&self) -> BoardIterator {
    Default::default()
  }
}

#[derive(Default)]
pub struct BoardIterator {
  _pos: Coords,
}

impl Iterator for BoardIterator {
  type Item = Coords;

  fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
    let pos = self._pos;
    if pos.y >= BOARD_SIZE {
      None
    } else if pos.x == pos.y {
      self._pos.x = 0;
      self._pos.y += 1;
      Some(pos)
    } else {
      self._pos.x += 1;
      Some(pos)
    }
  }
}

// forEach(cb: (coords: BoardCoords, value: ?T) => void) -> void {
//   for y in 0..self.size; y++) {
//     let row = self.__rows[y];
//     let rowSize = row.length;
//     for x in 0..rowSize; x++) {
//       let coords = { x, y };
//       let value = self.__rows[y][x];
//       cb(coords, value);
//     }
//   }
// }

// any(cb: (coords: BoardCoords, value: ?T) => bool) -> bool {
//   for y in 0..self.size; y++) {
//     let row = self.__rows[y];
//     let rowSize = row.length;
//     for x in 0..rowSize; x++) {
//       let coords = { x, y };
//       let value = self.__rows[y][x];
//       if (cb(coords, value)) return true;
//     }
//   }
//   return false;
// }

// every(cb: (coords: BoardCoords, value: ?T) => bool) -> bool {
//   for y in 0..self.size; y++) {
//     let row = self.__rows[y];
//     let rowSize = row.length;
//     for x in 0..rowSize; x++) {
//       let coords = { x, y };
//       let value = self.__rows[y][x];
//       if (!cb(coords, value)) return false;
//     }
//   }
//   return true;
// }

// pub fn map(cb: (coords: BoardCoords, value: ?T) =>?T) -> Board<T> {
//   let nextBoard = self.wipe();
//   self.forEach((pos, value) => {
//     nextBoard.__rows[pos.y][pos.x] = cb(pos, value);
//   });
//   return nextBoard;
// }
