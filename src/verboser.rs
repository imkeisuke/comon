use std::fmt;

pub trait Verboser: fmt::Debug {
    fn log(&self, msg: &str);
}

#[derive(Debug)]
pub struct SimpleVerboser;

impl Verboser for SimpleVerboser {
    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

#[derive(Debug)]
pub struct QuietVerboser;

impl Verboser for QuietVerboser {
    fn log(&self, _msg: &str) {}
}

pub fn create_verboser(verbose: bool) -> Box<dyn Verboser> {
    if verbose {
        Box::new(SimpleVerboser)
    } else {
        Box::new(QuietVerboser)
    }
}
