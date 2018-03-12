use std::vec::Vec;

use std::io::{Read, Write};

// Amount to be read by intermediate socket operations
const buff_size : usize = 1024;

pub struct Pipe<'a> {
    name : &'a str,
    buffer : Vec<char>,
}

pub struct PipeNetwork<'a> {
    unleased_pipes : Vec<Pipe<'a> >
}



pub fn pump(from : Read, to : Write) {
    let mut buff = [0 as u8; buff_size];
    let mut read = 1; // Any non-zero
    while read != 0 {
        read = from.read(&mut buff);
        to.write_all(&mut buff)
    }

    
}
