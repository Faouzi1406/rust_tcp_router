use std::collections::HashMap;
use crate::lib::{
    path::convert_to_path,
    status_codes::{STATUS_OK},
    path::match_route,
    path::patern_insert_from_path
};
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};


#[derive(Debug)]
pub struct RouteInfo {
    pub parent:Option<String>,
    pub params:Option<Vec<String>>,
    pub type_route:Option<String>,
    pub full_path:Option<String>
}

struct Route {
    path:RouteInfo,
    return_string:String
}

pub trait RouteParse {
    fn to_route(path:String) -> Self;
}

impl RouteInfo {
    pub fn new() -> RouteInfo{
        RouteInfo {
            parent:None,
            params:None,
            type_route:None,
            full_path:None
        }
    }
}

impl RouteParse for RouteInfo {
    fn to_route(path:String) -> Self {
        let string = path.split(" ");
        let mut route_info:Vec<String> =  Vec::new(); 
        
        for route_space in string {
            route_info.push(route_space.to_string());
        }

        let params = route_info[1].split("/");
        let mut params_in_route:Vec<String> = params
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

//
// impl Router for Vec<String> {
//  fn to_route(&self, dynamic_call_function:) -> Route {
//          let function:String  = dynamic_call_function();
//          todo!()
//}
//  }

impl RouteInfo {
    pub fn get(&self, path:&'static str,  stream:&mut TcpStream,  call_function:fn(params:HashMap<String, Option<String>>) -> String){
        let parent = &self.parent.clone().unwrap();

        let to_path = convert_to_path(path.to_string());

        if &to_path.parent == parent && 
        &self.type_route.clone() == &Some("GET".to_string()) && 
        match_route(self.full_path.to_owned().unwrap(), path.to_string())
        {
            let partern = patern_insert_from_path(path, self.params.clone().unwrap().to_owned());
            let function_call:String = call_function(partern);

            let length = function_call.len();
            let response =
            format!("{STATUS_OK}\r\nContent-Length: {length}\r\n\r\n{function_call}");

            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

