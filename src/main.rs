mod router;
mod lib;

use lib::{
    html_rust::HtmlHead,
    html_rust::HtmlBody,
    html_rust::Page
};

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use threadpool;

use std::collections::HashMap;
use router::{
    RouteInfo,
    RouteParse
};

#[tokio::main]
async fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to address");

    let thread_pool = threadpool::ThreadPool::new(4);

    while let Ok((stream, _)) = tcp_listener.accept(){
        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }
}


fn hello_route(params:HashMap<String, Option<String>>) -> String {
    format!("{:#?}", params.get("wow").unwrap().to_owned().unwrap())
}

fn some_route(params:HashMap<String, Option<String>>) -> String {
    let mut page:Page = Page::new();

    //Html head
    page.head.push(
        HtmlHead::TAG(format!("<title>Welkom op de web</title>"))
    );

    page.head.push(
        HtmlHead::TAG(format!("<link rel={} type={} href={}>", "icon", "image/x-icon", "https://web-dev.imgix.net/image/vS06HQ1YTsbMKSFTIPl2iogUQP73/KAOmqplghJT2PrJlOgZ5.png?auto=format"))
    );

    //Html body
    page.body.push(
        HtmlBody::P(format!("Dit is van de server {}", params.get("wow").unwrap().to_owned().unwrap()))
    );

    page.body.push(
        HtmlBody::P(format!("Cool toch?"))
    );

    page.create_page().0
}

fn hello(_params:HashMap<String, Option<String>>) -> String {
    format!("dit is een route")
}

fn handle_connection(mut stream:TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    

    if !http_request.is_empty(){
        let to_route:RouteInfo = RouteParse::to_route(http_request[0].to_owned());
        //Routes
        let _ = &to_route.get("hello/[wow]/cool", &mut stream, hello_route);
        let _ = &to_route.get("hello/[wow]/goodbye/[some]", &mut stream, some_route);
        let _ = &to_route.get("hello", &mut stream, hello);
    }

    stream.flush().expect("flushing stream error");
    stream.shutdown(std::net::Shutdown::Both).expect("Coulnd't shutdown");
    drop(stream);
}
