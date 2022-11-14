use crate::{
    logger::Logger,
    progress::{
        Bounded,
        Progress, // , BoundedProgress
        UnBounded,
        ESC,
    },
};

pub trait ProgressDisplay: Sized {
    fn display<'a, T: Iterator, L: Logger>(&self, progress: &Progress<'a, T, L, Self>) -> String;
}

impl ProgressDisplay for UnBounded {
    fn display<'a, T: Iterator, L: Logger>(&self, progress: &Progress<'a, T, L, Self>) -> String {
        match &progress.logger {
            Some(_) => format!("{}", "*".repeat(progress.i)),
            None => format!("{}", &format_args!("{}{}", ESC, "*".repeat(progress.i))),
        }
    }
}

impl ProgressDisplay for Bounded {
    fn display<'a, T: Iterator, L: Logger>(&self, progress: &Progress<'a, T, L, Self>) -> String {
        match &progress.logger {
            Some(_) => format!(
                "{}{}{}{}",
                self.delims.0,
                "*".repeat(progress.i),
                " ".repeat(self.bound - progress.i),
                self.delims.1
            ),
            None => format!(
                "{}",
                &format_args!(
                    "{}{}{}{}{}",
                    ESC,
                    self.delims.0,
                    "*".repeat(progress.i),
                    " ".repeat(self.bound - progress.i),
                    self.delims.1
                )
            ),
        }
    }
}
