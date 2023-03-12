use std::collections::HashSet;

const ROWS: i32 = 10;
const FILES: i32 = 10;

#[derive(Copy, Clone)]
pub struct Queen {
    location: Location,
}

pub struct Pawn {
    location: Location,
    colour: Colour,
}

impl HasLocation for Pawn {
    fn location(&self) -> Location {
        self.location
    }
}

impl HasMovement for Pawn {
    fn possible_moves(&self) -> HashSet<Location> {
        match self.colour {
            Colour::Black => black_pawn_moves(self.location),
            Colour::White => white_pawn_moves(self.location),
        }
    }
}

fn black_pawn_moves(location: Location) -> HashSet<Location> {
    let mut locations: HashSet<Location> = HashSet::new();
    locations.insert(Location::new(location.row - 1, location.file));
    if location.row == ROWS {
        locations.insert(Location::new(location.row - 2, location.file));
    }
    locations
}

fn white_pawn_moves(location: Location) -> HashSet<Location> {
    let mut locations: HashSet<Location> = HashSet::new();
    locations.insert(Location::new(location.row + 1, location.file));
    if (location.row == ROWS) {
        locations.insert(Location::new(location.row + 2, location.file));
    }
    locations
}

enum Colour {
    White,
    Black,
}

impl HasLocation for Queen {
    fn location(&self) -> Location {
        self.location
    }
}

impl HasMovement for Queen {
    fn possible_moves(&self) -> HashSet<Location> {
        let mut set = HashSet::new();
        let first: HashSet<Location> = (0..FILES)
            .map(|i| Location {
                row: self.location().row,
                file: i,
            })
            .collect();
        let second: HashSet<Location> = (0..ROWS)
            .map(|i| Location {
                row: i,
                file: self.location().file,
            })
            .collect();
        let diagonals = diagonal_positions(self.location);
        set.extend(first);
        set.extend(second);
        set.extend(diagonals);
        set
    }
}

fn diagonal_positions(start: Location) -> HashSet<Location> {
    let first = (-FILES..FILES + 1)
        .zip(-ROWS..ROWS + 1)
        .map(|(x, y)| Location {
            row: start.row + x,
            file: start.file + y,
        });
    let second = (-FILES..FILES + 1)
        .zip(ROWS..-ROWS - 1)
        .map(|(x, y)| Location {
            row: start.row + x,
            file: start.file + y,
        });
    first
        .chain(second)
        .into_iter()
        .filter(|loc| loc.row >= 0 && loc.row <= ROWS && loc.file >= 0 && loc.file <= FILES)
        .collect()
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Location {
    row: i32,
    file: i32,
}

impl Location {
    fn new(row: i32, file: i32) -> Location {
        Location {
            row: row,
            file: file,
        }
    }
}

trait HasMovement {
    fn possible_moves(&self, allPieces: dyn HashSet<HasLocation>) -> HashSet<Location>;
}

trait HasLocation {
    fn location(&self) -> Location;
}

#[test]
fn test_queen_movement() {
    let queen = Queen {
        location: Location { row: 0, file: 0 },
    };
    assert!(queen.possible_moves().contains(&Location::new(0, 0)));
    assert!(queen.possible_moves().contains(&Location::new(ROWS, 0)));
    assert!(queen.possible_moves().contains(&Location::new(ROWS, FILES)));
    assert!(queen.possible_moves().contains(&Location::new(0, FILES)));
}
