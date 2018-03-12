extern crate daemonize;
#[macro_use] extern crate log;

extern crate serde;
extern crate serde_json;

extern crate pipette;

use daemonize::{Daemonize};

//use unix_socket::{UnixStream, UnixListener};
use std::os::unix::net::{UnixStream, UnixListener};

use pipette::ipc::{ClientRequestHeader, DaemonResponse};

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
            Err(_) => {
                error!("Stream connection failed");
                break;
            }
        }
    }
}

fn handle_ipc_stream(stream : UnixStream) {
    // Get the message header
    let msg_header : ClientRequestHeader = serde_json::from_reader(stream).expect("Failed to parse pipette message header");

    // Operate based on it
    match msg_header {
        ClientRequestHeader::ReadFromSpout(name) => println!("{}", name),
        ClientRequestHeader::WriteToSink(name) => println!("{}", name),
        ClientRequestHeader::CreatePipePair(name) => println!("{}", name),
        ClientRequestHeader::DestroyPipePair(name) => println!("{}", name),
    }
}


