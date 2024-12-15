use crate::util::position::{Dimensions, Position};
use num::integer::div_rem;
use std::fmt::{Display, Formatter, Write};
use std::ops::{Index, IndexMut};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Grid<T> {
    pub dimensions: Dimensions,
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn from_dimensions(dimensions: Dimensions, value: T) -> Self
    where
        T: Clone,
    {
        Self {
            dimensions,
            data: vec![value; dimensions.0 * dimensions.1],
        }
    }
    pub fn from_rows<Rows, Cells>(rows: Rows) -> Self
    where
        Rows: IntoIterator<Item = Cells>,
        Cells: IntoIterator<Item = T>,
    {
        let mut data = vec![];
        let mut width = None;

        for row in rows {
            data.extend(row);
            width.get_or_insert(data.len());
        }

        let width = match width {
            None => {
                return Self {
                    dimensions: Dimensions(0, 0),
                    data,
                };
            }
            Some(width) => width,
        };

        assert_eq!(data.len() % width, 0);

        Self {
            dimensions: Dimensions(data.len() / width, width),
            data,
        }
    }

    pub fn size(&self) -> usize {
        self.dimensions.0 * self.dimensions.1
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (Position, &T)> + '_ {
        self.data
            .iter()
            .enumerate()
            .map(|(i, value)| (div_rem(i, self.dimensions.1).into(), value))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Position, &mut T)> + '_ {
        self.data
            .iter_mut()
            .enumerate()
            .map(|(i, value)| (div_rem(i, self.dimensions.1).into(), value))
    }

    pub fn values(&self) -> impl Iterator<Item = &T> + '_ {
        self.data.iter()
    }

    pub fn positions(&self) -> impl DoubleEndedIterator<Item = Position>
    where
        Position: From<(usize, usize)>,
    {
        let dimensions = self.dimensions;
        (0..self.data.len()).map(move |i| div_rem(i, dimensions.1).into())
    }

    pub fn positions_where(&self, mut f: impl FnMut(&T) -> bool) -> impl Iterator<Item = Position> {
        self.data
            .iter()
            .enumerate()
            .filter(move |&(_, value)| f(value))
            .map(|(i, _)| div_rem(i, self.dimensions.1).into())
    }

    fn index(&self, pos: &Position) -> usize {
        pos.0 * self.dimensions.1 + pos.1
    }

    pub fn get(&self, pos: &Position) -> &T {
        &self.data[self.index(pos)]
    }

    /// # Safety
    /// Position argument must be within boundaries.
    pub unsafe fn get_unchecked(&self, pos: &Position) -> &T {
        unsafe { self.data.get_unchecked(self.index(pos)) }
    }

    pub fn get_mut(&mut self, pos: &Position) -> &mut T {
        let i = self.index(pos);
        &mut self.data[i]
    }

    pub fn set(&mut self, pos: &Position, value: T) {
        let idx = self.index(pos);
        self.data[idx] = value;
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> + '_ {
        self.data.chunks_exact(self.dimensions.1)
    }

    pub fn get_row(&self, j: usize) -> &[T] {
        &self.data[j * self.dimensions.1..(j + 1) * self.dimensions.1]
    }

    pub fn transposed(&self) -> Self
    where
        T: Copy,
    {
        Grid::from_rows(
            (0..self.dimensions.1)
                .map(|i| (0..self.dimensions.0).map(move |j| self.data[j * self.dimensions.1 + i])),
        )
    }
}
impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.dimensions.1..(index + 1) * self.dimensions.1]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * self.dimensions.1..(index + 1) * self.dimensions.1]
    }
}

impl<A> Extend<((usize, usize), A)> for Grid<A> {
    fn extend<T: IntoIterator<Item = ((usize, usize), A)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(position, value)| {
            self[position.0][position.1] = value;
        });
    }
}

impl Grid<bool> {
    pub fn from_positions<I: Iterator<Item = Position> + Clone>(
        dimensions: Dimensions,
        iter: I,
    ) -> Self {
        let mut grid = Self::from_dimensions(dimensions, false);
        grid.extend(iter);
        grid
    }

    pub fn from_points<I: Iterator<Item = (usize, usize)> + Clone>(iter: I) -> Self {
        let mut max_x = usize::MIN;
        let mut max_y = usize::MIN;
        for (y, x) in iter.clone() {
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }

        let mut grid = Self::from_dimensions(Dimensions(max_y, max_x), false);
        grid.extend(iter.map(|pos| (pos, true)));
        grid
    }

    pub fn contains(&self, pos: &Position) -> bool {
        *self.get(pos)
    }

    pub fn insert(&mut self, pos: &Position) -> bool {
        let entry = self.get_mut(pos);
        if *entry {
            false
        } else {
            *entry = true;
            true
        }
    }

    pub fn remove(&mut self, pos: &Position) -> bool {
        let entry = self.get_mut(pos);
        if *entry {
            *entry = false;
            true
        } else {
            false
        }
    }

    pub fn extend_from_grid(&mut self, other: &Grid<bool>) {
        assert_eq!(self.dimensions, other.dimensions);
        self.data
            .iter_mut()
            .zip(other.data.iter())
            .for_each(|(elem, other)| {
                *elem |= *other;
            });
    }

    pub fn clear(&mut self) {
        self.data.iter_mut().for_each(|elem| *elem = false);
    }

    pub fn count(&self) -> usize {
        self.values().filter(|value| **value).count()
    }
}

pub trait CellDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

impl CellDisplay for bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            true => '█',
            false => ' ',
        })
    }
}

impl<T> Display for Grid<T>
where
    T: CellDisplay,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.data
            .chunks_exact(self.dimensions.1)
            .try_for_each(|row| {
                row.iter().try_for_each(|value| value.fmt(f))?;
                f.write_char('\n')
            })
    }
}

impl Extend<(usize, usize)> for Grid<bool> {
    fn extend<T: IntoIterator<Item = (usize, usize)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|position: (usize, usize)| {
            self[position.0][position.1] = true;
        });
    }
}

impl Extend<Position> for Grid<bool> {
    fn extend<I: IntoIterator<Item = Position>>(&mut self, iter: I) {
        iter.into_iter().for_each(|position| {
            self[position.y()][position.x()] = true;
        });
    }
}

pub struct BackedGrid<'a, I> {
    data: &'a [I],
    pub dimensions: (usize, usize),
    row_stride: usize,
}

impl<'a, I> BackedGrid<'a, I> {
    pub fn from_data_and_row_separator(data: &'a [I], separator: I) -> Self
    where
        I: Eq,
    {
        let width = data
            .iter()
            .position(|value| *value == separator)
            .unwrap_or(data.len());
        let row_stride = width + 1;
        Self {
            data,
            dimensions: (data.len().div_ceil(row_stride), width),
            row_stride,
        }
    }

    fn index(&self, pos: &Position) -> usize {
        pos.0 * self.row_stride + pos.1
    }

    pub fn get<T>(&self, pos: &Position) -> T
    where
        &'a I: Into<T>,
    {
        (&self.data[self.index(pos)]).into()
    }

    pub fn iter<T>(&'a self) -> impl Iterator<Item = (Position, T)>
    where
        &'a I: Into<T>,
    {
        self.data.iter().enumerate().filter_map(move |(i, value)| {
            let pair = div_rem(i, self.row_stride);
            if pair.1 >= self.dimensions.1 {
                None
            } else {
                Some((pair.into(), value.into()))
            }
        })
    }

    pub fn positions(&self) -> impl Iterator<Item = Position> {
        self.data.iter().enumerate().filter_map(move |(i, _)| {
            let pair = div_rem(i, self.row_stride);
            if pair.1 >= self.dimensions.1 {
                None
            } else {
                Some(Position(pair.0, pair.1))
            }
        })
    }
}

pub struct GridWindow3<'grid, T> {
    grid: &'grid Grid<T>,
    idx: usize,
}
impl<T> GridWindow3<'_, T> {
    pub fn center(&self) -> &T {
        &self.grid.data[self.idx]
    }
    pub fn top_left(&self) -> &T {
        &self.grid.data[self.idx - self.grid.dimensions.1 - 1]
    }
    pub fn top(&self) -> &T {
        &self.grid.data[self.idx - self.grid.dimensions.1]
    }
    pub fn top_right(&self) -> &T {
        &self.grid.data[self.idx - self.grid.dimensions.1 + 1]
    }
    pub fn right(&self) -> &T {
        &self.grid.data[self.idx + 1]
    }
    pub fn bottom_right(&self) -> &T {
        &self.grid.data[self.idx + self.grid.dimensions.1 + 1]
    }
    pub fn bottom(&self) -> &T {
        &self.grid.data[self.idx + self.grid.dimensions.1]
    }
    pub fn bottom_left(&self) -> &T {
        &self.grid.data[self.idx + self.grid.dimensions.1 - 1]
    }
    pub fn left(&self) -> &T {
        &self.grid.data[self.idx - 1]
    }
}

impl<T> Grid<T> {
    pub fn iter_windows3(&self) -> impl DoubleEndedIterator<Item = GridWindow3<'_, T>> + '_ {
        (1..self.dimensions.1 - 1).flat_map(move |y| {
            (1..self.dimensions.0 - 1).map(move |x| GridWindow3 {
                grid: self,
                idx: y * self.dimensions.1 + x,
            })
        })
    }

    pub fn iter_windows3_where(
        &self,
        mut f: impl FnMut(&T) -> bool,
    ) -> impl Iterator<Item = GridWindow3<'_, T>> {
        (self.dimensions.1 + 1..self.data.len() - self.dimensions.1 - 1).filter_map(move |i| {
            if !f(&self.data[i]) {
                return None;
            }
            let x = i % self.dimensions.1;

            (x != 0 && x != self.dimensions.1 - 1).then_some(GridWindow3 { grid: self, idx: i })
        })
    }
}

impl<T> Grid<T> {
    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = T>>
    where
        T: Clone,
    {
        (0..self.dimensions.1).map(move |x| {
            (0..self.dimensions.0)
                .map(move |y| self.get(&Position(y, x)))
                .cloned()
        })
    }
    pub fn diagonals_lower(&self) -> impl Iterator<Item = impl Iterator<Item = T>>
    where
        T: Clone,
    {
        let smallest = self.dimensions.0.min(self.dimensions.1);
        (0..smallest).map(move |y_start| {
            (0..smallest - y_start)
                .map(move |i| self.get(&Position(y_start + i, i)))
                .cloned()
        })
    }
    pub fn diagonals_upper(&self) -> impl Iterator<Item = impl Iterator<Item = T>>
    where
        T: Clone,
    {
        let smallest = self.dimensions.0.min(self.dimensions.1);
        (0..smallest).map(move |x_start| {
            (0..smallest - x_start)
                .map(move |i| self.get(&Position(i, x_start + i)))
                .cloned()
        })
    }
    pub fn anti_diagonals_upper(&self) -> impl Iterator<Item = impl Iterator<Item = T>>
    where
        T: Clone,
    {
        let smallest = self.dimensions.0.min(self.dimensions.1);
        (0..smallest).map(move |y_start| {
            (0..y_start + 1)
                .map(move |i| self.get(&Position(y_start - i, i)))
                .cloned()
        })
    }
    pub fn anti_diagonals_lower(&self) -> impl Iterator<Item = impl Iterator<Item = T>>
    where
        T: Clone,
    {
        let smallest = self.dimensions.0.min(self.dimensions.1);
        (0..smallest).map(move |x_start| {
            (0..smallest - x_start)
                .map(move |i| self.get(&Position(self.dimensions.0 - 1 - i, x_start + i)))
                .cloned()
        })
    }
}
