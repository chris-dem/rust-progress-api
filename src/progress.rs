use crate::logger::Logger;
use crate::progress_display::ProgressDisplay;
pub const ESC: &str = "\x1B[2J\x1B[1;1H";

#[derive(Clone, Copy)]
pub struct Bounded {
    pub bound: usize,
    pub delims: (char, char),
}

#[derive(Clone, Copy)]
pub struct UnBounded;

type Inp<'a, Log> = Option<&'a mut Log>;

pub struct Progress<'a, T, Log: Logger, B: ProgressDisplay> {
    pub iter: T,
    pub i: usize,
    pub logger: Option<&'a mut Log>,
    pub bounded: B,
}

// pub struct BoundedProgress<'a, T> {
//     pub progress: Progress<'a, T>,
//     pub bound: usize,
//     pub delims: (char, char),
//     // pub nums:
// }

impl<'a, T: ExactSizeIterator, Log: Logger> Progress<'a, T, Log, UnBounded> {
    pub fn with_bounds(self) -> Progress<'a, T, Log, Bounded> {
        Progress {
            bounded: Bounded {
                bound: self.iter.len(),
                delims: ('[', ']'),
            },
            i: self.i,
            iter: self.iter,
            logger: self.logger,
        }
    }
}

impl<'a, T: Iterator, Log: Logger> Progress<'a, T, Log, UnBounded> {
    pub fn new(iter: T, logger: Option<&'a mut Log>) -> Self {
        Progress {
            iter,
            i: 1,
            logger,
            bounded: UnBounded,
        }
    }
}
impl<'a, T: Iterator, Log: Logger> Progress<'a, T, Log, Bounded> {
    pub fn with_delims(mut self, delims: (char, char)) -> Self {
        self.bounded.delims = delims;
        self
    }
}

pub trait ProgressIteratorExt: Sized {
    fn progress<'a, Log: Logger>(self, logger: Inp<'a, Log>) -> Progress<'a, Self, Log, UnBounded>;
}

impl<T: Iterator> ProgressIteratorExt for T {
    fn progress<'a, L: Logger>(self, logger: Inp<'a, L>) -> Progress<'a, Self, L, UnBounded> {
        Progress::new(self, logger)
    }
}

impl<'a, T: Iterator, L: Logger, B: ProgressDisplay> Iterator for Progress<'a, T, L, B> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.iter.next();
        if res.is_some() {
            // self.logger.(self.bounded.bound());
            let st = self.bounded.display(&self);
            match &mut self.logger {
                Some(e) => e.print(st),
                None => println!("{}", st),
            };
            self.i += 1;
        }
        res
    }
}

// impl<'a, T: Iterator> Iterator for BoundedProgress<'a, T> {
//     type Item = T::Item;

//     fn next(&mut self) -> Option<Self::Item> {
//         let res = self.progress.iter.next();
//         if res.is_some() {
//             self.display();
//             self.progress.i += 1;
//         }
//         res
//     }
// }
