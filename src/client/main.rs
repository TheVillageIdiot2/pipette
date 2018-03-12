extern crate clap;
extern crate serde;
extern crate serde_json;

extern crate pipette;

use clap::{App, ArgGroup};

use std::os::unix::net::{UnixStream};
use std::io::{Read, Write, stdin, stdout};

use pipette::ipc::*;
use pipette::pipes::{pump};


enum ConnectionErrors {
    RequestDenied(String),
    ConnectionFailed,
}

fn main() {
    /* Read in args and send requests as necessary */
    let matches = App::new("Pipette Client")
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
        create_pipe(pipe_name);
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


fn init_connection(request : ClientRequestHeader) -> Result<UnixStream, ConnectionErrors> {
    // Connect to daemon
    if let Ok(mut stream) = UnixStream::connect("/tmp/pipette/ipc_socket") {
        //Send request
        send_request_header(request, &mut stream);

        // Get back response
        let response = read_response_header(&mut stream);
        match response {
            DaemonResponse::Confirm => Ok(stream),
            DaemonResponse::Deny(reason) => Err(ConnectionErrors::RequestDenied(reason))
        }
    } else {
        Err( ConnectionErrors::ConnectionFailed )
    }
}

fn init_or_fail(request : ClientRequestHeader) -> UnixStream {
    match init_connection(request) {
        Ok(stream) => stream,
        Err(err) => match err {
            ConnectionErrors::RequestDenied(reason) => panic!("Operation failed: {}", reason),
            ConnectionErrors::ConnectionFailed => panic!("Could not connect to Pipette daemon. Are you sure it is running?")
        }
    }
}

fn create_pipe(pipe_name : &str) {
    let _stream = init_or_fail(ClientRequestHeader::CreatePipePair(pipe_name.to_owned()));

    //We're actually done unless we failed, in which case we're also done
}

fn read_spout(pipe_name : &str) {
    // Connect to daemon
    let stream = init_or_fail(ClientRequestHeader::CreatePipePair(pipe_name.to_owned()));

    // Pump out of Daemon forever
    pump(stream, stdout);
    let out = stdout();

    stream.for_each(move |b| out.writej
}

fn write_sink(pipe_name : &str) {
    let _stream = init_or_fail(ClientRequestHeader::CreatePipePair(pipe_name.to_owned()));

}
