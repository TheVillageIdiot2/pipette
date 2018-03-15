use std::vec::Vec;

use std::io::{Read, Write};
use os_pipe::{pipe, PipeReader, PipeWriter};

// The tubes of our network
pub struct Pipette {
    name : String,
    pipe_sink : PipeWriter,
    pipe_spout : PipeReader
}

impl Pipette {
    fn new(name : String) -> Pipette {
        let (read, write) = pipe().unwrap();
        Pipette {
            name : name,
            pipe_sink  : write,
            pipe_spout : read
        }
    }

    fn feed(&mut self, x : u8) {
       self.pipe_sink.write(&[x]); 
    }
}

pub struct PipetteNetwork {
    unleased_pipes : Vec<Pipette>
}

/*
pub fn pump(from : &mut Read, to : &mut Write) {
    let mut read_bytes = 1; // Any non-zero
    let 
    while read != 0 {
        read = from.read(&mut buff);
        to.write_all(&mut buff)
    }
}
*/
