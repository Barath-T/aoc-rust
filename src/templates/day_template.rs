use std::fs;

pub struct Solve{
}

impl Solve{
  pub fn parse(&mut self){
    fs::read_to_string("input/dayx.txt");
  }
}
