use crate::logger::Logger;
use crate::progress_display::ProgressDisplay;
pub const ESC: &str = "\x1B[2J\x1B[1;1H";

type Inp<'a> = Option<&'a mut dyn Logger>;

#[derive(Clone, Copy)]
pub struct Bounded {
    pub bound: usize,
    pub delims: (char, char),
}

#[derive(Clone, Copy)]
pub struct UnBounded;

pub struct Progress<'a, T> {
    pub iter: T,
    pub i: usize,
    pub logger: Inp<'a>,
}

pub struct BoundedProgress<'a, T> {
    pub progress: Progress<'a, T>,
    pub bound: usize,
    pub delims: (char, char),
}

impl<'a, T: ExactSizeIterator> Progress<'a, T> {
    pub fn with_bounds(self) -> BoundedProgress<'a, T> {
        BoundedProgress {
            bound: self.iter.len(),
            delims: ('[', ']'),
            progress: self,
        }
    }
}

impl<'a, T: Iterator> Progress<'a, T> {
    pub fn new(iter: T, logger: Inp<'a>) -> Self {
        Progress { iter, i: 1, logger }
    }
}
impl<'a, T: Iterator> BoundedProgress<'a, T> {
    pub fn with_delims(mut self, delims: (char, char)) -> Self {
        self.delims = delims;
        self
    }
}

pub trait ProgressIteratorExt: Sized {
    fn progress<'a>(self, logger: Inp<'a>) -> Progress<'a, Self>;
}

impl<T: Iterator> ProgressIteratorExt for T {
    fn progress<'a>(self, logger: Inp<'a>) -> Progress<'a, Self> {
        Progress::new(self, logger)
    }
}

impl<'a, T: Iterator> Iterator for Progress<'a, T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.iter.next();
        if res.is_some() {
            self.display();
            self.i += 1;
        }
        res
    }
}

impl<'a, T: Iterator> Iterator for BoundedProgress<'a, T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.progress.iter.next();
        if res.is_some() {
            self.display();
            self.progress.i += 1;
        }
        res
    }
}
