mod routef;
mod tests;

use routef::{
    html_rust::HtmlBody,
    html_rust::HtmlHead,
    html_rust::{Page, CSS},
    router::{RouteInfo, RouteParse, RouteTypes},
    thread_pool::ThreadPool,
};

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use std::collections::HashMap;

fn main() {
    let tcp_listener = TcpListener::bind("0.0.0.0:8000").expect("Failed to bind to address");

    let thread_pool = ThreadPool::new(16);

    while let Ok((stream, _)) = tcp_listener.accept() {
        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn hello_route(params: HashMap<String, Option<String>>) -> String {
    format!("{:#?}", params.get("wow").unwrap().to_owned().unwrap())
}

fn index(_params: HashMap<String, Option<String>>) -> String {
    let some_value = "hello!";

    format!("Dit is de index route")
}

fn four_0_four(_params: HashMap<String, Option<String>>) -> String {
    format!("Deze pagina bestaat niet :(")
}

fn some_route(params: HashMap<String, Option<String>>) -> String {
    let mut page: Page = Page::new();

    page.head = vec!(
        HtmlHead::TAG(format!("<title>Welkom op de web</title>")),
        HtmlHead::TAG(format!("<link rel={} type={} href={}>", "icon", "image/x-icon", "https://web-dev.imgix.net/image/vS06HQ1YTsbMKSFTIPl2iogUQP73/KAOmqplghJT2PrJlOgZ5.png?auto=format"))
    );

    page.body = vec![
        HtmlBody::H1("Hello".to_string()),
        HtmlBody::P(format!("Dit is van de server {}", params.get("wow").unwrap().to_owned().unwrap())),
        HtmlBody::P(format!("Cool toch?")),
        HtmlBody::Img(routef::html_rust::Img {
            src:"https://99designs-blog.imgix.net/blog/wp-content/uploads/2022/06/Starbucks_Corporation_Logo_2011.svg-e1657703028844.png?auto=format&q=60&fit=max&w=930".to_string(),
            width:400,
            height:400
        }),
        HtmlBody::FileWithProps("public/index.html".to_string(), params.to_owned()),
        HtmlBody::P(format!("some params: {:#?}", params))
    ];

    page.css = vec![CSS::File("public/style.css".to_string())];

    page.create_page().0
}

fn test_route(params: HashMap<String, Option<String>>) -> String {
    let mut page: Page = Page::new();

    page.head = vec![HtmlHead::TAG(format!("<title>Welkom op de web</title>"))];

    page.body = vec![HtmlBody::FileWithProps(
        "public/test.html".to_string(),
        params.to_owned(),
    )];

    page.create_page().0
}

fn hello(_params: HashMap<String, Option<String>>) -> String {
    format!("dit is een route")
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if !http_request.is_empty() {
        let to_route: RouteInfo = RouteParse::to_route(http_request[0].to_owned());
        //Get routes
        let _ = &to_route.get("hello/[wow]/cool", &mut stream, hello_route);
        let _ = &to_route.get("hello/[wow]/goodbye/[some]", &mut stream, some_route);
        let _ = &to_route.get("hello", &mut stream, hello);
        let _ = &to_route.get("test/route/[test]", &mut stream, test_route);
        let _ = &to_route.get("", &mut stream, index);

        //Every route should come before this  fall back route
        let _ = &to_route.get("*", &mut stream, four_0_four);

        //Post routes
        let _ = &to_route.post("posting", &mut stream, index);
    }

    stream.flush().expect("flushing stream error");
}
