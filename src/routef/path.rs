use std::collections::HashMap;
#[derive(Debug)]
pub struct Path {
    pub parent:String,
    pub params:Vec<String>
}

impl Path {
    fn new() -> Self{
        Path {
            parent:String::new(),
            params:Vec::new()
        }
    }
}

pub fn convert_to_path(path:String) -> Path {
    let mut return_path:Path = Path::new();
    let path = path.split("/");

    return_path.parent = path.clone().nth(0).unwrap().to_string();

    for param in  path.skip(1){
        return_path.params.push(param.to_string());
    }
    
    return_path
}

pub fn patern_insert_from_path(
    path:&'static str, 
    params:Vec<String>) -> HashMap<String,Option<String>> {
    let mut route_params:HashMap<String, Option<String>> = HashMap::new();

    let path_params  = path.split("/");

    let mut param_vec:Vec<String> = Vec::new();

    for param in params {
        if param != ""{
            param_vec.push(param.to_owned());
        }
    }

    for (i, path_param) in path_params.enumerate() {
        let param = path_param.replace("[", "").replace("]", "");
        route_params.insert(param, Some(param_vec.get(i).unwrap().to_owned()));
    };

    route_params
}

pub fn match_route(path:String, incoming_path:String) -> bool {
    let mut path_find = path.clone();
    path_find.remove(0);
    let mut is_matched = path_find.matches("/").count() == incoming_path.matches("/").count();
    let path_params:Vec<&str> = path_find
        .split("/")
        .collect();
    let inc = incoming_path.split("/");
    
    if is_matched{
    for (i, param) in inc.enumerate() {
         let get_param = path_params[i];
         if !param.contains("]") && !param.contains("["){
            if get_param != param {
                is_matched = false;
                break;
            }
         }
    }
    }

    is_matched
}
