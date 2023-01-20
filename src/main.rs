mod router;
mod lib;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

use std::collections::HashMap;
use router::{
    RouteInfo,
    RouteParse
};

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:8000");
    let listenr = tcp_listener.unwrap();

    for stream in listenr.incoming(){
        let stream_inc = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream_inc);
        });
    }
}

fn hello_route(params:HashMap<String, Option<String>>) -> String {
    format!("{:#?}", params.get("wow").unwrap())
}


fn some_route(params:HashMap<String, Option<String>>) -> String {
    format!("{:#?}: {:#?}", params.get("some").unwrap(), params.get("wow").unwrap())
}

fn hello(_params:HashMap<String, Option<String>>) -> String {
    format!("dit is een route")
}

fn handle_connection(mut stream:TcpStream) {
    let buf_reader:BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let connection:Vec<String> = buf_reader
        .lines()
        .map(|x| x.unwrap())
        .take_while(|x| !x.is_empty())
        .collect();

    let to_route:RouteInfo = RouteParse::to_route(connection[0].to_owned());

    //Routes
    let _ = &to_route.get("hello/[wow]/cool", &mut stream, hello_route);
    let _ = &to_route.get("hello/[wow]/goodbye/[some]", &mut stream, some_route);
    let _ = &to_route.get("hello", &mut stream, hello);
}
