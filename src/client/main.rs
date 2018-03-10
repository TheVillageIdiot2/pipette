extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate unix_socket;

extern crate pipette;

use clap::{Arg, App, ArgGroup};

use std::io::Write;

use pipette::ipc::{DaemonResponse, ClientRequestHeader};

fn main() {
    /* Read in args and send requests as necessary */
    let matches = App::with_defaults("Pipette Client")
        // Args for creating and destroying pipes
        .args_from_usage(
            "--create [name]    'Create a new pipe-pair'
             --destroy [name]   'Destroy an existing pipe-pair'
             --list             'List all existing pipes'")
        .group(ArgGroup::with_name("manage")
               .args(&["create", "destroy", "list"]))

        // Args for accessing pipes
        .args_from_usage(
            "--sink [name]      'Open the sink of an existing pipe-pair, and take input till EOF'
             --spout [name]     'Open up the spout of an existing pipe-pair, and output till EOF'")
        .group(ArgGroup::with_name("access")
               .args(&["sink", "spout"])
               .conflicts_with("manage"))

        .get_matches();

    //Create case
    if let Some(pipe_name) = matches.value_of("create") {
        //if the_pipe_DNE
        println!("Creating pipe {}", pipe_name);
    }
    
    //Read case
    else if let Some(pipe_name) = matches.value_of("spout") {
        //if the_pipe_exists
        println!("Reading continuously from spout {}", pipe_name);
        read_spout(pipe_name);
    } 

    //Write case
    else if let Some(pipe_name) = matches.value_of("sink") {
        //if the_pipe_exists
        println!("Reading continuously from spout {}", pipe_name);
        write_sink(pipe_name);
    }
}

enum ConnectionErrors {
    RequestDenied(DaemonResponse::Deny),
    ConnectionFailed,
}

fn init_connection(request : ClientRequestHeader) -> Result<unix_socket::UnixStream, ConnectionErrors> {
    // Convert to serialized message
    let serialized = serde_json::to_string(&request).expect("Failed to serialize request header");

    // Connect to daemon
    if let Ok(mut stream) = unix_socket::UnixStream::connect("/tmp/pipette/ipc_socket") {
        // Send our request
        stream.write_all(serialized.as_bytes()).expect("Failed to send request header");

        // Get back response
        let response : DaemonResponse = serde_json::from_reader(stream).expect("Failed to deserialize response header");
        match response {
            DaemonResponse::Confirm => Ok(stream),
            d @ DaemonResponse::Deny => Err(ConnectionErrors::RequestDenied(d))
        }
    } else {
        Err( ConnectionErrors::ConnectionFailed )
    }
}

fn create_pipe(pipe_name : &str) {

}

fn read_spout(pipe_name : &str) {
    let msg = ClientRequestHeader::ReadFromSpout(pipe_name.to_owned());
}

fn write_sink(pipe_name : &str) {

}

fn print_no_socket() {
    println!("Could not connect to pipette. Are you sure the daemon is running?")
}
