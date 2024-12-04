#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RotationalDirection {
    Clockwise,
    Anticlockwise,
}

impl RotationalDirection {
    pub fn from_incoming_and_outgoing(incoming: &Direction, outgoing: &Direction) -> Option<Self> {
        match (*outgoing as u8 + 4 - *incoming as u8) % 4 {
            0 => None,
            1 => Some(RotationalDirection::Clockwise),
            2 => None,
            3 => Some(RotationalDirection::Anticlockwise),
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    #[must_use]
    pub fn with_rotation(self, rotational_direction: &RotationalDirection) -> Self {
        match (self, rotational_direction) {
            (Direction::Up, RotationalDirection::Clockwise) => Direction::Right,
            (Direction::Up, RotationalDirection::Anticlockwise) => Direction::Left,
            (Direction::Down, RotationalDirection::Clockwise) => Direction::Left,
            (Direction::Down, RotationalDirection::Anticlockwise) => Direction::Right,
            (Direction::Right, RotationalDirection::Clockwise) => Direction::Down,
            (Direction::Right, RotationalDirection::Anticlockwise) => Direction::Up,
            (Direction::Left, RotationalDirection::Clockwise) => Direction::Up,
            (Direction::Left, RotationalDirection::Anticlockwise) => Direction::Down,
        }
    }

    #[must_use]
    pub fn inverted(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct PositionOffset(pub isize, pub isize);

impl From<(isize, isize)> for PositionOffset {
    fn from(value: (isize, isize)) -> Self {
        PositionOffset(value.0, value.1)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Dimensions(pub usize, pub usize);
impl Dimensions {
    pub fn height(&self) -> usize {
        self.0
    }
    pub fn width(&self) -> usize {
        self.1
    }
}

impl From<Dimensions> for (usize, usize) {
    fn from(value: Dimensions) -> Self {
        (value.height(), value.width())
    }
}

impl From<(usize, usize)> for Dimensions {
    fn from(value: (usize, usize)) -> Self {
        Dimensions(value.0, value.1)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Position(pub usize, pub usize);

impl Position {
    pub fn from_yx(y: usize, x: usize) -> Self {
        Self(y, x)
    }
    pub fn y(&self) -> usize {
        self.0
    }
    pub fn x(&self) -> usize {
        self.1
    }

    fn can_move(&self, dimensions: &Dimensions, direction: &PositionOffset) -> bool {
        self.0.wrapping_add_signed(direction.0) < dimensions.0
            && self.1.wrapping_add_signed(direction.1) < dimensions.1
    }

    pub fn checked_moved(&self, dimensions: &Dimensions, offset: &PositionOffset) -> Option<Self> {
        if self.can_move(dimensions, offset) {
            Some(Self(
                self.0.checked_add_signed(offset.0).unwrap(),
                self.1.checked_add_signed(offset.1).unwrap(),
            ))
        } else {
            None
        }
    }

    pub fn checked_move(&mut self, dimensions: &Dimensions, offset: &PositionOffset) -> bool {
        if self.can_move(dimensions, offset) {
            self.0 = self.0.checked_add_signed(offset.0).unwrap();
            self.1 = self.1.checked_add_signed(offset.1).unwrap();
            true
        } else {
            false
        }
    }

    pub fn moved(&self, offset: &PositionOffset) -> Self {
        Self(
            self.0.checked_add_signed(offset.0).unwrap(),
            self.1.checked_add_signed(offset.1).unwrap(),
        )
    }

    pub fn positions(
        &self,
        dimensions: &Dimensions,
        offset: &PositionOffset,
    ) -> impl Iterator<Item = Position> {
        let dimensions = *dimensions;
        let mut current = *self;
        std::iter::from_fn(move || {
            if current.checked_move(&dimensions, offset) {
                Some(current)
            } else {
                None
            }
        })
    }

    pub fn manhattan_distance(&self, other: &Position) -> usize {
        let num = if self.0 < other.0 {
            other.0 - self.0
        } else {
            self.0 - other.0
        };
        num + if self.1 < other.1 {
            other.1 - self.1
        } else {
            self.1 - other.1
        }
    }
}

impl From<Position> for (usize, usize) {
    fn from(value: Position) -> Self {
        (value.y(), value.x())
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Position(value.0, value.1)
    }
}
