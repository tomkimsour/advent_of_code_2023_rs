#![allow(incomplete_features)]
#![allow(unused_imports)]

use derive_more::{Deref, DerefMut};
use itertools::Itertools;
use std::iter::{self};
use std::ops::Neg;
use std::{
    array::from_fn,
    fmt::Debug,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Deref, DerefMut)]
pub struct Point<const N: usize>(pub [i64; N]);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Deref, DerefMut)]
pub struct Vector<const N: usize>(pub [i64; N]);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Deref, DerefMut)]
pub struct Cell<const N: usize>(pub [i64; N]);

pub type Point2 = Point<2>;
pub type Vector2 = Vector<2>;
pub type Cell2 = Cell<2>;

#[must_use]
pub fn c2(x: i64, y: i64) -> Cell<2> {
    Cell::new([x, y])
}

impl<const N: usize> Default for Cell<N> {
    fn default() -> Self {
        Self(from_fn(|_| 0))
    }
}

impl<const N: usize> Cell<N> {
    pub fn origin() -> Self {
        Self(from_fn(|_| 0))
    }

    pub fn new(x: [i64; N]) -> Self {
        Self(x)
    }

    pub fn vector(&self) -> Vector<N> {
        *self - Self::origin()
    }

    // pub fn point(&self) -> Point<N> {
    //     Point::new(self.0)
    // }

    // pub fn adj(&self) -> [Self; N * 2] {
    //     Vector::<N>::adj().map(|x| x + *self)
    // }

    // pub fn adj_diag(&self) -> [Self; count_adj_diag(N)] {
    //     Vector::<N>::adj_diag().map(|x| x + *self)
    // }

    // pub fn corners(&self) -> [Point<N>; count_corners(N)] {
    //     from_fn(|i| {
    //         let mut point = self.point();
    //         for n in 0..N {
    //             if i & (1 << n) != 0 {
    //                 point.0[n] += 1;
    //             }
    //         }
    //         point
    //     })
    // }

    // pub fn corners_minmax(&self) -> [Point<N>; 2] {
    //     [self.point(), self.point() + Vector::new(from_fn(|_| 1))]
    // }
}

impl<const N: usize> Add<Cell<N>> for Vector<N> {
    type Output = Cell<N>;

    fn add(self, rhs: Cell<N>) -> Self::Output {
        Cell(from_fn(|i| self.0[i] + rhs.0[i]))
    }
}

impl<const N: usize> Add<Vector<N>> for Cell<N> {
    type Output = Self;

    fn add(self, rhs: Vector<N>) -> Self::Output {
        Self(from_fn(|i| self.0[i] + rhs.0[i]))
    }
}

impl<const N: usize> Sub<Vector<N>> for Cell<N> {
    type Output = Self;

    fn sub(self, rhs: Vector<N>) -> Self::Output {
        Self(from_fn(|i| self.0[i] - rhs.0[i]))
    }
}

impl<const N: usize> Sub for Cell<N> {
    type Output = Vector<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(from_fn(|i| self.0[i] - rhs.0[i]))
    }
}

impl<const N: usize> AddAssign<Vector<N>> for Cell<N> {
    fn add_assign(&mut self, rhs: Vector<N>) {
        *self = *self + rhs;
    }
}

impl<const N: usize> SubAssign<Vector<N>> for Cell<N> {
    fn sub_assign(&mut self, rhs: Vector<N>) {
        *self = *self - rhs;
    }
}

// impl<const N: usize> Cartesian<N> for Cell<N> {
//     fn inner(&self) -> [i64; N] {
//         self.0
//     }

//     fn new(x: [i64; N]) -> Self {
//         Self(x)
//     }
// }

impl<const N: usize> Debug for Cell<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cell[{}]", self.0.iter().join(", "))
    }
}
