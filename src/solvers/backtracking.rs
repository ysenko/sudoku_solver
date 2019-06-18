extern crate log;

use std::fmt;

const SQUARE_SIDE: usize = 3;
const SIDE: usize = SQUARE_SIDE * 3;
const SIZE: usize = SIDE * SIDE;
const EMPTY: u8 = 0;

struct LogEntry {
    pos: usize,
    val: u8,
}

#[derive(Debug, Clone)]
struct ValueNotAllowed {
    pub msg: String,
}

#[derive(Debug, Clone)]
pub struct Unsolvable {}

pub struct Sudoku {
    field: [u8; SIZE],
    backtrack_log: Vec<LogEntry>,
}

impl Sudoku {
    /// Builds a new sudoku from the provided field.
    pub fn new(field: impl Iterator<Item = u8>) -> Option<Sudoku> {
        debug!("Creating a new sudoku");
        let field: Vec<u8> = field.collect();
        match field.len() {
            SIZE => {
                let mut field_array: [u8; SIZE] = [EMPTY; SIZE];
                field_array.copy_from_slice(&field);
                Some(Sudoku {
                    field: field_array,
                    backtrack_log: Vec::new(),
                })
            }
            _ => None,
        }
    }

    /// Returns true if the given number is allowed in row on the given position.
    fn is_allowed_in_row(&self, number: u8, pos: usize) -> bool {
        let y = pos / SIDE;
        !(0..SIDE)
            .map(|i| self.field[y * SIDE + i])
            .any(|el| el == number)
    }

    /// Returns true if the given number is allowed in column on the given position.
    fn is_allowed_in_col(&self, number: u8, pos: usize) -> bool {
        let x = pos % SIDE;
        !(0..SIDE)
            .map(|i| self.field[i * SIDE + x])
            .any(|el| el == number)
    }

    /// Returns true if the given number is allowed in square on the given position.
    fn is_allowed_in_square(&self, number: u8, pos: usize) -> bool {
        let y = pos / SIDE;
        let x = pos % SIDE;
        let square_start_x = (x / SQUARE_SIDE) * SQUARE_SIDE;
        let square_start_y = (y / SQUARE_SIDE) * SQUARE_SIDE;
        !(0..SIDE)
            .map(|i| {
                let square_x = i % SQUARE_SIDE + square_start_x;
                let square_y = i / SQUARE_SIDE + square_start_y;
                self.field[SIDE * square_y + square_x]
            })
            .any(|el| el == number)
    }

    /// Returns true if the given element is allowed on a given position.
    fn is_allowed(&self, number: u8, pos: usize) -> bool {
        // Make sure value is not already set and is valid.
        if self.field[pos] != EMPTY || number > SIDE as u8 {
            return false;
        }
        vec![
            self.is_allowed_in_col(number, pos),
            self.is_allowed_in_row(number, pos),
            self.is_allowed_in_square(number, pos),
        ]
        .iter()
        .all(|el| el.to_owned())
    }

    /// Set the value of the given position.
    ///
    /// The function tries to set the value of the given position and returns a result if value is allowed there.
    /// Otherwise the function will return an error.
    /// Each successful call to the function is tracked in the log, and the last call van be rolled back with the
    /// `self.rollback()` call.
    fn set_value(&mut self, number: u8, pos: usize) -> Result<(), ValueNotAllowed> {
        if !self.is_allowed(number, pos) {
            return Err(ValueNotAllowed {
                msg: "Value is not allowed in the position".to_string(),
            });
        }
        self.field[pos] = number;
        self.backtrack_log.push(LogEntry {
            pos: pos,
            val: number,
        });
        debug!("Value {} set for position {}", number, pos);
        Ok(())
    }

    /// Try to fill the position with values from `start` to 9.
    ///
    /// Return Ok() if position filled with some value, otherwise None.
    fn fill_position(&mut self, pos: usize, start: u8) -> Option<()> {
        for val in start..SIDE as u8 + 1 {
            match self.set_value(val, pos) {
                Ok(_) => return Some(()),
                Err(_) => {}
            }
        }
        None
    }

    /// Rollback the most recent set action.
    ///
    /// Returns error when rollback log is empty.
    fn rollback(&mut self) -> Result<LogEntry, ()> {
        match self.backtrack_log.pop() {
            None => Err(()),
            Some(action) => {
                debug!("Rollback for position {}", action.pos);
                self.field[action.pos] = EMPTY;
                Ok(action)
            }
        }
    }

    /// Returns a position of a next empty cell or None if all all cells are filled.
    fn next_empty(&self) -> Option<usize> {
        for i in 0..SIZE {
            if self.field[i] == EMPTY {
                return Some(i);
            }
        }
        None
    }

    /// Returns `true` if sudoku is solved, otherwise `false`.
    pub fn solved(&self) -> bool {
        self.field.iter().all(|i| i != &EMPTY)
    }

    pub fn solve(&mut self) -> Result<(), Unsolvable> {
        // Return solved if there are no empty cells.
        let mut pos = self.next_empty();
        let mut start_val = 1;

        loop {
            let pos_idx = match pos {
                Some(v) => v,
                _ => break,
            };
            match self.fill_position(pos_idx, start_val) {
                None => {
                    match self.rollback() {
                        Ok(log_entry) => {
                            pos = Some(log_entry.pos);
                            start_val = log_entry.val + 1;
                            // continue
                        }
                        Err(_) => {
                            // Nothing to rollback. Sudoku is unsolvable.
                            break;
                        }
                    }
                }
                Some(_) => {
                    start_val = 1;
                    pos = self.next_empty()
                }
            }
        }
        match self.solved() {
            true => Ok(()),
            false => Err(Unsolvable {}),
        }
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", "=====================================")?;
        for i in 0..SIDE {
            for j in 0..SIDE {
                write!(
                    f,
                    "| {} ",
                    match self.field[SIDE * i + j] {
                        0 => " ".to_string(),
                        v => v.to_string(),
                    }
                )?;
            }
            write!(f, "|\n")?;
            match i == SIDE - 1 || (i != 0 && i % 3 == 2) {
                false => writeln!(f, "{}", "|-----------|-----------|-----------|")?,
                true => writeln!(f, "{}", "=====================================")?,
            }
        }
        Ok(())
    }
}

#[test]
fn new_sudoku() {
    let mut field: Vec<u8> = vec![9; SIZE];
    field[0] = 0;
    let sudoku = Sudoku::new(field.into_iter()).unwrap();
    let mut res: String = "".to_string();
    let expected_out = concat!(
        "=====================================\n",
        "|   | 9 | 9 | 9 | 9 | 9 | 9 | 9 | 9 |\n",
        "|-----------|-----------|-----------|\n",
        "| 9 | 9 | 9 | 9 | 9 | 9 | 9 | 9 | 9 |\n",
        "|-----------|-----------|-----------|\n",
        "| 9 | 9 | 9 | 9 | 9 | 9 | 9 | 9 | 9 |\n",
        "=====================================\n",
        "| 9 | 9 | 9 | 9 | 9 | 9 | 9 | 9 | 9 |\n",
        "|-----------|-----------|-----------|\n",
        "| 9 | 9 | 9 | 9 | 9 | 9 | 9 | 9 | 9 |\n",
        "|-----------|-----------|-----------|\n",
        "| 9 | 9 | 9 | 9 | 9 | 9 | 9 | 9 | 9 |\n",
        "=====================================\n",
        "| 9 | 9 | 9 | 9 | 9 | 9 | 9 | 9 | 9 |\n",
        "|-----------|-----------|-----------|\n",
        "| 9 | 9 | 9 | 9 | 9 | 9 | 9 | 9 | 9 |\n",
        "|-----------|-----------|-----------|\n",
        "| 9 | 9 | 9 | 9 | 9 | 9 | 9 | 9 | 9 |\n",
        "=====================================\n"
    );

    write!(&mut res, "{}", sudoku).unwrap();

    assert_eq!(&expected_out, &res);
}

fn test_field_helper() -> Sudoku {
    let mut field: Vec<u8> = vec![0; SIZE];
    // First square if filled except of central cell. Allowed value is 5.
    field[0] = 1;
    field[1] = 2;
    field[2] = 3;
    field[9] = 4;
    field[11] = 6;
    field[18] = 7;
    field[19] = 8;
    field[20] = 9;
    // First row is filled except of cell #3. Allowed value is 4.
    field[4] = 5;
    field[5] = 6;
    field[6] = 7;
    field[7] = 8;
    field[8] = 9;
    // First col is filled except of cell #3. Allowed value is 2.
    field[36] = 3;
    field[45] = 5;
    field[54] = 6;
    field[63] = 8;
    field[72] = 9;

    Sudoku::new(field.into_iter()).unwrap()
}

fn solvable_field_helper() -> Sudoku {
    let field: Vec<u8> = vec![
        5, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0, 0,
        0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0, 0, 0,
        2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
    ];
    Sudoku::new(field.into_iter()).unwrap()
}

#[test]
fn not_allowed_in_square() {
    let s = test_field_helper();
    assert!(!s.is_allowed(1, 10));
}

#[test]
fn allowed_in_square() {
    let s = test_field_helper();
    assert!(s.is_allowed(5, 10));
}

#[test]
fn allowed_in_row() {
    let s = test_field_helper();
    assert!(s.is_allowed(4, 3));
}

#[test]
fn not_allowed_in_row() {
    let s = test_field_helper();
    assert!(!s.is_allowed(5, 3));
}

#[test]
fn allowed_in_col() {
    let s = test_field_helper();
    assert!(s.is_allowed(2, 27));
}

#[test]
fn not_allowed_in_col() {
    let s = test_field_helper();
    assert!(!s.is_allowed(4, 27));
}

#[test]
fn set_value() {
    let mut s = test_field_helper();
    assert!(s.set_value(2, 27).is_ok());
}

#[test]
fn set_value_which_is_already_set() {
    let mut s = test_field_helper();
    assert!(s.set_value(2, 1).is_err());
}

#[test]
fn rollback() {
    let mut s = test_field_helper();
    s.set_value(2, 27).is_ok();
    let log = s.rollback().unwrap();
    assert_eq!(27, log.pos);
    assert_eq!(2, log.val);
}

#[test]
fn rollback_with_empty_log() {
    let mut s = test_field_helper();
    assert!(s.rollback().is_err());
}

#[test]
fn next_empty() {
    let s = test_field_helper();
    assert_eq!(3, s.next_empty().unwrap());
}

#[test]
fn next_empty_on_solved_field() {
    let s = Sudoku::new(vec![9; SIZE].into_iter()).unwrap();
    assert!(s.next_empty().is_none());
}

#[test]
fn fill_position() {
    let mut s = test_field_helper();
    assert!(s.fill_position(3, 1).is_some());
    assert_eq!(4, s.field[3]);
}

#[test]
fn fill_position_9_is_a_valid_choice() {
    let field: Vec<u8> = vec![
        5, 3, 1, 2, 7, 4, 8, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0, 6, 0, 8, 0, 0,
        0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0, 6, 0, 6, 0, 0, 0, 0,
        2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
    ];
    let mut s = Sudoku::new(field.into_iter()).unwrap();

    assert!(s.fill_position(7, 1).is_some());
    assert_eq!(s.field[7], 9);
}

#[test]
fn fill_unfillable_position() {
    let mut s = test_field_helper();
    assert!(s.fill_position(3, 5).is_none());
    assert_eq!(0, s.field[3]);
}

#[test]
fn not_solved() {
    let s = solvable_field_helper();
    assert!(!s.solved());
}

#[test]
fn solved() {
    let s = Sudoku::new(vec![9; SIZE].into_iter()).unwrap();
    assert!(s.solved());
}

#[test]
fn solve() {
    let mut s = solvable_field_helper();
    let res = s.solve();
    assert!(res.is_ok());
    assert!(s.solved());
}

#[test]
fn solve_empty() {
    let mut s = Sudoku::new(vec![0; SIZE].into_iter()).unwrap();
    assert!(s.solve().is_ok());
    assert!(s.solved());
}
