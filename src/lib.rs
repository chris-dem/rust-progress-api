use core::fmt;
use std::{time::Duration,thread::sleep, fmt::Arguments, rc::Rc, cell::{RefCell, Ref, RefMut}, borrow::BorrowMut};

pub const ESC :&str = "\x1B[2J\x1B[1;1H";


trait Logger {
    fn print(&self, value : &Arguments<'_>);
}

struct StringLogger(RefCell<String>);

impl StringLogger {
    pub fn new(st : String) -> Self {
        StringLogger(RefCell::new(st))
    }

    pub fn borrow_string(&self) -> Ref<'_, String> {
        self.0.borrow()
    } 
    
    pub fn mut_borrow_string(&self) -> RefMut<'_, String> {
        self.0.borrow_mut()
    } 
}

impl Logger for StringLogger {
    fn print(&self, value : &Arguments<'_>) {
        self.0.borrow_mut().push_str(format!("{}",value).as_str());
    }
}

type Inp<'a> = Option<&'a dyn Logger>;

#[derive(Clone, Copy)]
struct UnBounded;

#[derive(Clone, Copy)]
struct Bounded {
    bound : usize,
    delims : (char,char)
}

struct Progress<'a ,T,B : ProgressDisplay> {
    iter : T,
    i : usize,
    logger : Inp<'a>,
    bound : B,
}

impl <'a,T : ExactSizeIterator> Progress<'a,T,UnBounded> {
    pub fn with_bounds(self) -> Progress<'a,T,Bounded>{
        Progress {
            bound : Bounded {
                bound :self.iter.len(),
                delims : ('[',']')
            },
            i : self.i,
            iter : self.iter,
            logger : self.logger
        }
    }
}

trait ProgressDisplay : Sized {
    fn display<T>(&self, progress : &Progress<'_,T,Self>);
}

impl ProgressDisplay for UnBounded {
   fn display<T>(&self,progress : &Progress<'_,T,Self>) {
        match &progress.logger {
            Some(e)  => e.print(&format_args!("{}{}", ESC,"*".repeat(progress.i))),
            None => println!("{}",&format_args!("{}{}", ESC,"*".repeat(progress.i))),
        };
    }
}

impl ProgressDisplay for Bounded {
    fn display<T>(&self, progress : &Progress<'_,T,Self>) {
    match &progress.logger {
            Some(e)  => e.print(&format_args!("{}{}{}{}{}", ESC,self.delims.0,"*".repeat(progress.i), " ".repeat(self.bound - progress.i),self.delims.1)),
            None =>println!("{}",&format_args!("{}{}{}{}{}", ESC,self.delims.0,"*".repeat(progress.i), " ".repeat(self.bound - progress.i),self.delims.1)),
    };
  }
}

impl <'a,T : Iterator> Progress<'a,T,UnBounded> {
    pub fn new(iter : T, logger : Inp<'a> ) -> Self {
        Progress { iter , i : 1, logger, bound : UnBounded }
    }

}
impl <'a,T : Iterator> Progress<'a,T,Bounded> {
    pub fn with_delims(mut self, delims : (char,char)) -> Self {
        self.bound.delims = delims;
        self
    }

 
}

impl <'a,T : Iterator> Progress<'a,T,UnBounded> {

    
}



impl <'a,T : Iterator,B : ProgressDisplay> Iterator for Progress<'a,T,B> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.iter.next();
        if res.is_some() {
            self.bound.display(&self);
            self.i += 1;
        }
        res
    }
}

trait ProgressIteratorExt : Sized {
    fn progress<'a>(self, logger : Inp<'a>) -> Progress<'a,Self,UnBounded>;
}

impl<T:Iterator> ProgressIteratorExt for T {
    fn progress<'a>(self, logger : Inp<'a>) -> Progress<'a,Self,UnBounded> {
        Progress::new(self,logger)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::*;
    fn exp_foo(_n : &i32) {
        sleep(Duration::from_secs(1));
    }


    #[test] 
    fn it_works() {
        let inp  = vec![1,2,3,4];
        // let out_str = String::from_str(ESC).unwrap() + (&(inp.iter().map(|x :&i32| "*".repeat(*x as usize)).collect::<Vec<String>>().join(ESC)));
        let sl = StringLogger::new("".to_owned()); 
        // for i in inp.iter().progress(None).with_bounds().with_delims(('{','}')) {
        //     //
        //     exp_foo(i);
        // }
        for i in (1..).progress(None) {
            //
            exp_foo(&i);
        }
        println!("{}", sl.borrow_string());
    }
}
