#[cfg(test)]
mod tests{
    use std::collections::HashMap;

    use crate::routef::html_rust::{Page, HtmlHead, HtmlBody};

    #[test]
    fn test_parser_ok(){
        let mut params:HashMap<String, Option<String>> = HashMap::new();
        params.insert("test".to_string(), Some("cool%20toch".to_string()));

        let mut page = Page::new();
        page.head = vec![
            HtmlHead::TAG("<title>Welkom op de web</title>".to_string())
        ];
        page.body = vec![
            HtmlBody::FileWithProps("public/test.html".to_string(), params)
        ];
        
        let page = page.create_page();    
        assert_eq!(page.0, "<html>
<head>
<title>Welkom op de web</title> 
</head> 
 <body>
<div>
  dit is een test: cool%20toch
</div>
</body>
 <style></style> 
</html>");
    }


    #[test]
    fn test_parser_fail(){
        let mut params:HashMap<String, Option<String>> = HashMap::new();
        params.insert("test".to_string(), Some("this prop is wrong".to_string()));

        let mut page = Page::new();
        page.head = vec![
            HtmlHead::TAG("<title>Welkom op de web</title>".to_string())
        ];
        page.body = vec![
            HtmlBody::FileWithProps("public/test.html".to_string(), params)
        ];
        
        let page = page.create_page();    
        assert_ne!(page.0, "<html>
<head>
<title>Welkom op de web</title> 
</head> 
 <body>
<div>
  dit is een test: cool%20toch
</div>
</body>
 <style></style> 
</html>");
    }
}
