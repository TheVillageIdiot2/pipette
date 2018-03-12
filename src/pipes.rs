use std::vec::Vec;

pub struct Pipe<'a> {
    name : &'a str,
    buffer : Vec<char>,
}

pub struct PipeNetwork<'a> {
    unleased_pipes : Vec<Pipe<'a> >
}
