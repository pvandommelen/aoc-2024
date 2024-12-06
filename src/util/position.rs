use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RotationalDirection {
    Clockwise,
    Anticlockwise,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct PositionOffset(pub isize, pub isize);

impl PositionOffset {
    pub fn up() -> Self {
        PositionOffset(-1, 0)
    }
    pub fn right() -> Self {
        PositionOffset(0, 1)
    }
    pub fn down() -> Self {
        PositionOffset(1, 0)
    }
    pub fn left() -> Self {
        PositionOffset(0, -1)
    }

    #[must_use]
    pub fn rotated(self, rotational_direction: &RotationalDirection) -> Self {
        match rotational_direction {
            RotationalDirection::Clockwise => Self(self.1, -self.0),
            RotationalDirection::Anticlockwise => Self(-self.1, self.0),
        }
    }

    #[must_use]
    pub fn inverted(&self) -> Self {
        Self(-self.0, -self.1)
    }
}

impl From<(isize, isize)> for PositionOffset {
    fn from(value: (isize, isize)) -> Self {
        PositionOffset(value.0, value.1)
    }
}

impl Mul<isize> for &PositionOffset {
    type Output = PositionOffset;

    fn mul(self, rhs: isize) -> Self::Output {
        PositionOffset(self.0 * rhs, self.1 * rhs)
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
        assert!(offset.0 != 0 || offset.1 != 0);

        let steps_y = if offset.0 == 0 {
            isize::MAX
        } else if offset.0.is_positive() {
            (dimensions.0 - 1 - self.0) as isize / offset.0
        } else {
            self.0 as isize / -offset.0
        };
        let steps_x = if offset.1 == 0 {
            isize::MAX
        } else if offset.1.is_positive() {
            (dimensions.1 - 1 - self.1) as isize / offset.1
        } else {
            self.1 as isize / -offset.1
        };
        let steps = steps_y.min(steps_x);

        (0..steps).map(move |i| *self + offset * (i + 1))
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

impl Add<PositionOffset> for Position {
    type Output = Position;

    fn add(self, rhs: PositionOffset) -> Self::Output {
        Position(
            self.0.checked_add_signed(rhs.0).unwrap(),
            self.1.checked_add_signed(rhs.1).unwrap(),
        )
    }
}
impl AddAssign<PositionOffset> for Position {
    fn add_assign(&mut self, rhs: PositionOffset) {
        self.0 = self.0.checked_add_signed(rhs.0).unwrap();
        self.1 = self.1.checked_add_signed(rhs.1).unwrap();
    }
}
