extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate unix_socket;

extern crate pipette;

use clap::{Arg, App};

use std::io::Write;

use pipette::ipc::{DaemonResponse, ClientRequestHeader};

fn main() {
    let matches = App::new("Pipette Client")
        .arg(Arg::with_name("read_spout")
             .short("r")
             .long("read")
             .value_name("PIPENAME"))
        .get_matches();

    if let Some(pipe_name) = matches.value_of("read_spout") {
        //if the_pipe_exists
        println!("Reading continuously from spout {}", pipe_name);
    }


}

fn read_spout(name : String) {
    let msg = ClientRequestHeader::ReadFromSpout(name);

    // Convert to serialized message
    let serialized = serde_json::to_string(msg).unwrap();

    // Connect to daemon
    if let Ok(stream) = unix_socket::UnixStream::connect("/tmp/pipette/ipc_socket") {
        // Send our request
        stream.write_all(serialized.as_bytes());

        // Get back response
        let response : DaemonResponse = serde_json::from_reader(stream);
        match response {
            _ => ()
        };
    } else {
        print_no_socket();
    }
}

fn print_no_socket() {
    println!("Could not connect to pipette. Are you sure the daemon is running?")
}
