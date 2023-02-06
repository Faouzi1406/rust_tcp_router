use std::{collections::HashMap, os::unix::prelude::FileExt, fs::File}; 
use crate::routef::{
    path::convert_to_path,
    status_codes::STATUS_OK,
    path::match_route,
    path::patern_insert_from_path
};
use std::{
    io::prelude::*,
    net::TcpStream,
};


#[derive(Debug)]
pub struct RouteInfo {
    pub parent:Option<String>,
    pub params:Option<Vec<String>>,
    pub type_route:Option<String>,
    pub full_path:Option<String>
}


pub trait RouteParse {
    fn to_route(path:String) -> Self;
}

impl RouteParse for RouteInfo {
    fn to_route(path:String) -> Self {
        let string = path.split(" ");
        let mut route_info:Vec<String> =  Vec::new(); 
        
        for route_space in string {
            route_info.push(route_space.to_string());
        }

        let params = route_info[1].split("/");
        let params_in_route:Vec<String> = params
            .map(|x| x.to_string())
            .collect();

        RouteInfo {
            parent:Some(params_in_route[1].to_owned()),
            params:Some(params_in_route),
            type_route:Some(route_info[0].to_owned()),
            full_path:Some(route_info[1].to_owned())
        }
    }
}



impl RouteInfo {
    pub fn response(&self, path:&'static str,  stream:&mut TcpStream,  call_function:fn(params:HashMap<String, Option<String>>) -> String, response_type:String){
        let parent = &self.parent.clone().unwrap();

        let to_path = convert_to_path(path.to_string());

        if &to_path.parent == parent && 
        &self.type_route.clone() == &Some(response_type.to_string()) && 
        match_route(self.full_path.to_owned().unwrap(), path.to_owned())
        {
            let partern = patern_insert_from_path(path, self.params.clone().unwrap().to_owned());
            let function_call:String = call_function(partern);

            let length = function_call.len();
            let response =
                format!("{STATUS_OK}\r\nContent-Type: text/html\r\nContent-Length: {length}\r\n\r\n{function_call}");

            stream.write(response.as_bytes()).unwrap();
        }
        else if path == "*" && &self.type_route.clone() == &Some(response_type) {
            let partern = patern_insert_from_path(path, self.params.clone().unwrap().to_owned());
            let function_call:String = call_function(partern);

            let length = function_call.len();
            let response =
                format!("{STATUS_OK}\r\nContent-Type: text/html\r\nContent-Length: {length}\r\n\r\n{function_call}");

            stream.write(response.as_bytes()).unwrap();
        }
    }
}

pub trait RouteTypes {
    fn get(&self, path:&'static str,  stream:&mut TcpStream,  call_function:fn(params:HashMap<String, Option<String>>) -> String);
    fn post(&self, path:&'static str,  stream:&mut TcpStream,  call_function:fn(params:HashMap<String, Option<String>>) -> String);
    fn put(&self, path:&'static str,  stream:&mut TcpStream,  call_function:fn(params:HashMap<String, Option<String>>) -> String);
    fn delete(&self, path:&'static str,  stream:&mut TcpStream,  call_function:fn(params:HashMap<String, Option<String>>) -> String);
    fn patch(&self, path:&'static str,  stream:&mut TcpStream,  call_function:fn(params:HashMap<String, Option<String>>) -> String);
}

impl RouteTypes for RouteInfo {
    fn get(&self, path:&'static str,  stream:&mut TcpStream,  call_function:fn(params:HashMap<String, Option<String>>) -> String){
        self.response(path, stream, call_function, "GET".to_string());
    }

    fn post(&self, path:&'static str,  stream:&mut TcpStream,  call_function:fn(params:HashMap<String, Option<String>>) -> String){
        self.response(path, stream, call_function, "POST".to_string());
    }

    fn put(&self, path:&'static str,  stream:&mut TcpStream,  call_function:fn(params:HashMap<String, Option<String>>) -> String){
        self.response(path, stream, call_function, "PUT".to_string());
    }

    fn delete(&self, path:&'static str,  stream:&mut TcpStream,  call_function:fn(params:HashMap<String, Option<String>>) -> String){
        self.response(path, stream, call_function, "DELETE".to_string());
    }

    fn patch(&self, path:&'static str,  stream:&mut TcpStream,  call_function:fn(params:HashMap<String, Option<String>>) -> String){
        self.response(path, stream, call_function, "PATCH".to_string());
    }
}

pub trait RouteFiles {
    fn serve_file(&self, path:&'static str,  stream:&mut TcpStream,  call_function:fn(params:HashMap<String, Option<String>>) -> Option<File>);
}
