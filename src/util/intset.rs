use std::fmt::Debug;

#[derive(Clone, Default)]
pub struct IntSet {
    data: Vec<u64>,
}

pub fn iter_int_slices<T>(data: &[u64]) -> impl Iterator<Item = T> + '_
where
    T: Copy + From<usize>,
{
    data.iter().enumerate().flat_map(|(index, &val)| {
        (0..64).filter_map(move |offset| {
            if val & (1 << offset) != 0 {
                Some((index * 64 + offset).into())
            } else {
                None
            }
        })
    })
}

impl IntSet {
    pub fn new() -> IntSet {
        IntSet { data: Vec::new() }
    }

    pub fn with_maximum<T>(maximum: T) -> IntSet
    where
        T: Into<usize>,
    {
        IntSet {
            data: vec![0; (maximum.into() + 63) / 64],
        }
    }

    pub fn insert<T>(&mut self, val: T) -> bool
    where
        T: Copy + Into<usize>,
    {
        let val = val.into();
        let index = val / 64;
        let offset = val % 64;
        if index >= self.data.len() {
            self.data.resize(index + 1, 0);
        }
        let cur = self.data[index];
        self.data[index] |= 1 << offset;
        (cur & 1 << offset) == 0
    }

    pub fn contains<T>(&self, val: &T) -> bool
    where
        T: Copy + Into<usize>,
    {
        let val = (*val).into();
        let index = val / 64;
        let offset = val % 64;
        if index >= self.data.len() {
            return false;
        }
        (self.data[index] & (1 << offset)) != 0
    }

    pub fn iter<T>(&self) -> impl Iterator<Item = T>
    where
        T: Copy + From<usize>,
    {
        iter_int_slices(&self.data)
    }

    pub fn len(&self) -> usize {
        self.data.iter().map(|val| val.count_ones() as usize).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.data.iter().all(|val| val.count_ones() == 0)
    }

    pub fn clear(&mut self) {
        self.data.iter_mut().for_each(|val| {
            *val = 0;
        });
    }
}

impl<I> FromIterator<I> for IntSet
where
    I: Copy + Into<usize>,
{
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        let mut set = IntSet { data: Vec::new() };
        for item in iter {
            set.insert(item);
        }
        set
    }
}

impl Debug for IntSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "IntSet(len={})",
            self.data.iter().map(|val| val.count_ones()).sum::<u32>()
        ))
    }
}

#[derive(Copy, Clone)]
pub struct ArraySet64<const N: usize>
where
    [u64; N]: Copy,
{
    data: [u64; N],
}

impl<const N: usize> ArraySet64<N>
where
    [u64; N]: Copy,
{
    pub fn new() -> ArraySet64<N> {
        ArraySet64 { data: [0; N] }
    }

    pub fn insert<T>(&mut self, val: T)
    where
        T: Into<usize>,
    {
        let val: usize = val.into();
        assert!(val < N * 64);
        let index = val / 64;
        let offset = val % 64;
        self.data[index] |= 1 << offset;
    }

    pub fn contains<T>(&self, val: &T) -> bool
    where
        T: Copy + Into<usize>,
    {
        let val = (*val).into();
        assert!(val < N * 64);
        let index = val / 64;
        let offset = val % 64;
        (self.data[index] & (1 << offset)) != 0
    }

    pub fn iter<T>(&self) -> impl Iterator<Item = T> + '_
    where
        T: Copy + From<usize>,
    {
        iter_int_slices(&self.data)
    }

    pub fn len(&self) -> usize {
        self.data.iter().map(|val| val.count_ones() as usize).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.data.iter().all(|val| val.count_ones() == 0)
    }
}
impl<const N: usize> Default for ArraySet64<N>
where
    [u64; N]: Copy,
{
    fn default() -> Self {
        Self::new()
    }
}
