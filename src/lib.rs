mod day;
pub mod template;
pub mod defaultmap;
pub mod cartesian;

pub use std::fmt::Display;
pub use crate::defaultmap::*;
pub use crate::cartesian::*;

pub use day::*;

impl<T: Display> DisplayExt for T {}

pub trait DisplayExt: Display {
    fn digits(&self) -> Vec<u32> {
        self.to_string().chars().filter_map(|x| x.to_digit(10)).collect()
    }
}

pub trait ExtraItertools: IntoIterator + Sized {
    fn ii(self)-> <Self as IntoIterator>::IntoIter{
        self.into_iter()
    }
}
impl<T: IntoIterator + Sized> ExtraItertools for T {}
