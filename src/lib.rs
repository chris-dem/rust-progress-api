pub mod logger;
pub mod number;
pub mod progress;
pub mod progress_display;

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use crate::{progress::ProgressIteratorExt, *};

    fn _exp_foo(_n: &i32) {
        sleep(Duration::from_secs(1));
    }

    mod bounded {
        use super::*;
        #[test]
        fn it_works_bounded_with_bounds() {
            let it = vec![1, 2, 3, 4, 5, 6];
            let out_str = it
                .iter()
                .map(|x| {
                    format!(
                        "[{}{}]",
                        "*".repeat(*x as usize),
                        " ".repeat(it.len() - *x as usize)
                    )
                })
                .collect::<Vec<String>>()
                .join("");

            let mut slog = logger::StringLogger::new("".to_owned());

            let _ = it
                .iter()
                .progress(Some(&mut slog))
                .with_bounds()
                .collect::<Vec<&i32>>();
            assert_eq!(out_str, slog.0);
        }

        #[test]
        fn it_works_bounded() {
            let it = vec![1, 2, 3, 4, 5, 6];
            let out_str = it
                .iter()
                .map(|x| format!("{}", "*".repeat(*x as usize)))
                .collect::<Vec<String>>()
                .join("");

            let mut slog = logger::StringLogger::new("".to_owned());

            let _ = it.iter().progress(Some(&mut slog)).collect::<Vec<&i32>>();
            assert_eq!(out_str, slog.0);
        }

        #[test]
        fn it_works_bounded_with_bounds_with_delims() {
            let it = vec![1, 2, 3, 4, 5, 6];
            let out_str = it
                .iter()
                .map(|x| {
                    format!(
                        "<{}{}>",
                        "*".repeat(*x as usize),
                        " ".repeat(it.len() - *x as usize)
                    )
                })
                .collect::<Vec<String>>()
                .join("");

            let mut slog = logger::StringLogger::new("".to_owned());

            let _ = it
                .iter()
                .progress(Some(&mut slog))
                .with_bounds()
                .with_delims(('<', '>'))
                .collect::<Vec<&i32>>();
            assert_eq!(out_str, slog.0);
        }
    }
    mod unbounded {
        use super::*;
        #[test]
        fn it_works_unbounded_with_bounds() {
            let out_str = (1..)
                .map(|x| format!("{}", "*".repeat(x as usize),))
                .take(6)
                .collect::<Vec<String>>()
                .join("");

            let mut slog = logger::StringLogger::new("".to_owned());

            let _ = (1..)
                .progress(Some(&mut slog))
                .take(6)
                .collect::<Vec<i32>>();
            assert_eq!(out_str, slog.0);
        }
    }
}
