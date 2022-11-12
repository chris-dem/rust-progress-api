use crate::progress::{
      Progress
    , BoundedProgress
    // , UnBounded
    // , Bounded
    , ESC
};

pub trait ProgressDisplay : Sized  {

    fn display(&mut self);

}





impl<'a,T : Iterator> ProgressDisplay for Progress<'a,T> {
   fn display(&mut self) {
        match &mut self.logger {
            Some(e)  => e.print(&format_args!("{}","*".repeat(self.i))),
            None => println!("{}",&format_args!("{}{}", ESC,"*".repeat(self.i))),
        };
    }
}

impl<'a,T : Iterator> ProgressDisplay for BoundedProgress<'a,T>{


    fn display(&mut self) {
    match &mut self.progress.logger {
            Some(e)  => e.print(&format_args!("{}{}{}{}", self.delims.0,"*".repeat(self.progress.i), " ".repeat(self.bound - self.progress.i),self.delims.1)),
            None =>println!("{}",&format_args!("{}{}{}{}{}", ESC,self.delims.0,"*".repeat(self.progress.i), " ".repeat(self.bound - self.progress.i),self.delims.1)),
    };
  }
}


