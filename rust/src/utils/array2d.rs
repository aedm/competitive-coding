use itertools::Itertools;
use std::ops::{Add, Index, IndexMut, Mul, Neg, Rem};
use derive_more::{AddAssign, DivAssign, MulAssign, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, AddAssign, SubAssign, MulAssign, DivAssign)]
pub struct IVec2D {
    pub x: i64,
    pub y: i64,
}

pub const DIRS4: &'static [IVec2D] = &[
    IVec2D { x: 0, y: -1 }, // north
    IVec2D { x: -1, y: 0 }, // west
    IVec2D { x: 0, y: 1 },  // south
    IVec2D { x: 1, y: 0 },  // east
];

pub const DIRS8: &'static [IVec2D] = &[
    IVec2D { x: 0, y: -1 }, // north
    IVec2D { x: -1, y: -1 },
    IVec2D { x: -1, y: 0 }, // west
    IVec2D { x: -1, y: 1 },
    IVec2D { x: 0, y: 1 }, // south
    IVec2D { x: 1, y: 1 },
    IVec2D { x: 1, y: 0 }, // east
    IVec2D { x: 1, y: -1 },
];

pub fn v(x: i64, y: i64) -> IVec2D {
    IVec2D::new(x, y)
}

impl IVec2D {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn map_add<T>(&self, d: IVec2D, map: &Map2D<T>) -> Option<Self> {
        let p = Self::new(self.x + d.x, self.y + d.y);
        (p.x >= 0 && p.x < map.w && p.y >= 0 && p.y < map.h).then_some(p)
    }
}

impl Neg for IVec2D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl Mul<IVec2D> for IVec2D {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Mul<i64> for IVec2D {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Add<IVec2D> for IVec2D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<(i64, i64)> for IVec2D {
    type Output = Self;
    fn add(self, rhs: (i64, i64)) -> Self::Output {
        Self::new(self.x + rhs.0, self.y + rhs.1)
    }
}

impl Rem<IVec2D> for IVec2D {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Self::new(self.x % rhs.x, self.y % rhs.y)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct Map2D<T> {
    pub items: Vec<T>,
    pub w: i64,
    pub h: i64,
}

impl Map2D<char> {
    pub fn from_text(text: &[String]) -> Self {
        let w = text[0].len() as i64;
        let h = text.len() as i64;
        let items = text.iter().flat_map(|l| l.chars()).collect::<Vec<_>>();
        Self { items, w, h }
    }
}

impl Map2D<u8> {
    pub fn from_text_u8(text: &[String]) -> Self {
        let w = text[0].len() as i64;
        let h = text.len() as i64;
        let items = text.iter().flat_map(|l| l.bytes()).collect::<Vec<_>>();
        Self { items, w, h }
    }
}

impl<T: Clone> Map2D<T> {
    pub fn from_map<K>(map: &Map2D<K>, f: impl Fn(&K) -> T) -> Self {
        Self {
            items: map.items.iter().map(f).collect(),
            w: map.w,
            h: map.h,
        }
    }

    pub fn new(w: i64, h: i64, item: T) -> Self {
        Self {
            items: vec![item; (w * h) as usize],
            w,
            h,
        }
    }

    pub fn from_fn(w: i64, h: i64, f: impl Fn(IVec2D) -> T) -> Self {
        Self {
            items: (0..w * h).map(|i| f(IVec2D::new(i % w, i / w))).collect(),
            w,
            h,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (IVec2D, T)> + '_ {
        Itertools::cartesian_product(0..self.h, 0..self.w)
            .map(move |(y, x)| (IVec2D::new(x, y), self[IVec2D::new(x, y)].clone()))
    }

    pub fn filter<F: Fn(IVec2D, &T) -> bool>(&self, f: F) -> impl Iterator<Item = (IVec2D, &T)> {
        Itertools::cartesian_product(0..self.h, 0..self.w)
            .map(move |(y, x)| (IVec2D::new(x, y), &self[IVec2D::new(x, y)]))
            .filter(move |&(p, t)| f(p, t))
    }

    pub fn add_coord(&self, c: IVec2D, d: IVec2D) -> Option<(IVec2D, T)> {
        let p = c + d;
        (p.x >= 0 && p.x < self.w && p.y >= 0 && p.y < self.h).then(|| (p, self[p].clone()))
    }

    pub fn size(&self) -> IVec2D {
        IVec2D::new(self.w, self.h)
    }
}

impl<T> Index<IVec2D> for Map2D<T> {
    type Output = T;
    fn index(&self, index: IVec2D) -> &Self::Output {
        &self.items[(index.y * self.w + index.x) as usize]
    }
}

impl<T> IndexMut<IVec2D> for Map2D<T> {
    fn index_mut(&mut self, index: IVec2D) -> &mut Self::Output {
        &mut self.items[(index.y * self.w + index.x) as usize]
    }
}
