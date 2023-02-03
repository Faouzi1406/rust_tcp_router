#[cfg(test)]
mod tests {
    use crate::routef::router::{RouteParse, RouteInfo};

    #[test] 
    fn test_parent(){
        let some_path = "GET /home/some/home HTTP/1.1";
        let route:RouteInfo= RouteParse::to_route(some_path.to_string());
        assert_eq!(route.parent.unwrap(), "home");
    }

    #[test] 
    fn test_type(){
        let some_path = "GET /home/some/home HTTP/1.1";
        let route:RouteInfo = RouteParse::to_route(some_path.to_string());
        assert_eq!(route.type_route.unwrap(), "GET");
    }


    #[test] 
    fn test_params(){
        let some_path = "GET /home/some/home HTTP/1.1";
        let route:RouteInfo = RouteParse::to_route(some_path.to_string());
        assert_eq!(route.params.unwrap(), vec!["","home","some", "home"]);
    }

    #[test]
    fn test_path(){
        let some_path = "GET /home/some/home HTTP/1.1";
        let route:RouteInfo = RouteParse::to_route(some_path.to_string());
        assert_eq!(route.full_path.unwrap(), "/home/some/home");
    }

    #[test]
    fn test_path_with_params(){
        let some_path = "GET /home/some/home?name=John&age=20 HTTP/1.1";
        let route:RouteInfo = RouteParse::to_route(some_path.to_string());
        assert_eq!(route.full_path.unwrap(), "/home/some/home");
    }
}
