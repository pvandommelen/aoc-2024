use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

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

    pub fn moved(&self, direction: &Direction) -> Self {
        self.offset(&direction.into())
    }

    pub fn checked_moved(&self, dimensions: &Dimensions, direction: &Direction) -> Option<Self> {
        self.checked_offset(dimensions, &direction.into())
    }

    pub fn offset(&self, offset: &PositionOffset) -> Self {
        Self(
            self.0.wrapping_add_signed(offset.0),
            self.1.wrapping_add_signed(offset.1),
        )
    }

    pub fn checked_offset(&self, dimensions: &Dimensions, offset: &PositionOffset) -> Option<Self> {
        let (y, y_overflow) = self.0.overflowing_add_signed(offset.0);
        let (x, x_overflow) = self.1.overflowing_add_signed(offset.1);
        if y_overflow || x_overflow || y >= dimensions.0 || x >= dimensions.1 {
            None
        } else {
            Some(Self(y, x))
        }
    }

    pub fn wrapping_offset(&self, dimensions: &Dimensions, offset: &PositionOffset) -> Self {
        let offset_wrapped = (
            dimensions.0.wrapping_add_signed(
                offset.0.wrapping_add_unsigned(self.0) % dimensions.0 as isize,
            ),
            dimensions.1.wrapping_add_signed(
                offset.1.wrapping_add_unsigned(self.1) % dimensions.1 as isize,
            ),
        );
        Self(
            offset_wrapped.0 % dimensions.0,
            offset_wrapped.1 % dimensions.1,
        )
    }

    pub fn positions(
        &self,
        dimensions: &Dimensions,
        direction: &Direction,
    ) -> impl Iterator<Item = Position> + use<> + Clone {
        let steps = match direction {
            Direction::Up => self.0,
            Direction::Right => dimensions.1 - self.1 - 1,
            Direction::Down => dimensions.0 - self.0 - 1,
            Direction::Left => self.1,
        };

        let offset: PositionOffset = direction.into();
        let s = *self;
        (1..steps as isize + 1).map(move |i| {
            Position::from_yx(
                s.0.wrapping_add_signed(offset.0 * i),
                s.1.wrapping_add_signed(offset.1 * i),
            )
        })
    }

    pub fn positions_steps(
        &self,
        dimensions: &Dimensions,
        offset: &PositionOffset,
    ) -> impl Iterator<Item = Position> + Clone {
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

        let s = *self;
        (1..steps + 1).map(move |i| s + offset * i)
    }

    pub fn manhattan_distance(&self, other: &Position) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}
struct StepIterator {
    current: isize,
    step: isize,
    target: isize,
}
impl StepIterator {
    fn new(init: usize, target: usize, step: isize) -> Self {
        Self {
            current: init as isize,
            step,
            target: target as isize,
        }
    }
}
impl Iterator for StepIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.target {
            None
        } else {
            self.current += self.step;
            Some(self.current as usize)
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
impl Sub<Position> for Position {
    type Output = PositionOffset;

    fn sub(self, rhs: Position) -> Self::Output {
        PositionOffset(
            self.0 as isize - rhs.0 as isize,
            self.1 as isize - rhs.1 as isize,
        )
    }
}
impl Div<isize> for PositionOffset {
    type Output = PositionOffset;

    fn div(self, rhs: isize) -> Self::Output {
        PositionOffset(self.0 / rhs, self.1 / rhs)
    }
}
impl Neg for PositionOffset {
    type Output = PositionOffset;

    fn neg(self) -> Self::Output {
        PositionOffset(-self.0, -self.1)
    }
}
impl Mul<isize> for PositionOffset {
    type Output = PositionOffset;

    fn mul(self, rhs: isize) -> Self::Output {
        PositionOffset(rhs * self.0, rhs * self.1)
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}
pub const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

impl Direction {
    #[must_use]
    pub fn rotated(self, rotational_direction: &RotationalDirection) -> Self {
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

impl From<&Direction> for PositionOffset {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Up => PositionOffset(-1, 0),
            Direction::Right => PositionOffset(0, 1),
            Direction::Down => PositionOffset(1, 0),
            Direction::Left => PositionOffset(0, -1),
        }
    }
}
