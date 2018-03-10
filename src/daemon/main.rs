extern crate daemonize;
extern crate unix_socket;
#[macro_use] extern crate log;

extern crate serde;
extern crate serde_json;

use daemonize::{Daemonize};

use unix_socket::{UnixStream, UnixListener};

use std::thread;

fn main() {
    println!("Hello world!");
    let daemonic = Daemonize::new()
        .pid_file("/tmp/pipette.pid")
        .chown_pid_file(true)
        .working_directory("/tmp/pipette/")
        .user("nobody")
        .group("daemon");

    match daemonic.start() {
        Ok(_) => {
            info!("Success, pipette demon started");
            ipc_loop()
        },
        Err(e) => error!("{}", e),
    }
}

fn ipc_loop() {
    // Listen at a file
    let listener = UnixListener::bind("/tmp/pipette/ipc_socket").unwrap();

    // Accept connections and process them 
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_ipc_stream(stream));
            }
            Err(err) => {
                error!("Stream connection failed");
                break;
            }
        }
    }
}

fn handle_ipc_stream(stream : UnixStream) {
}
