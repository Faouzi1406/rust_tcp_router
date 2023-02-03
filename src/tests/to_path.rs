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
}
