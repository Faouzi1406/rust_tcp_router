pub enum HtmlHead { 
    TAG(String),
}

pub struct Img {
    pub src:String, 
    pub width:i32,
    pub height:i32
}

pub enum HtmlBody {
    H1(String),
    P(String),
    Img(Img)
}

pub struct HtmlString(pub String);

pub struct Page {
    pub head:Vec<HtmlHead>,
    pub body:Vec<HtmlBody>
}


impl Page {
    pub fn new() -> Page {
        Page{
        head:Vec::new(),
        body:Vec::new()
        }
    }
    
    pub fn parse_head(&self) -> HtmlString {
        let mut headtags = format!("\n<head>\n");
        
        for tag in &self.head {
            match tag { 
                HtmlHead::TAG(value) => {headtags.push_str(format!("{value} \n").as_str())}
            }
        };

        headtags.push_str("</head>");
        HtmlString(headtags)
    }

    pub fn parse_body(&self) -> HtmlString {
        let mut body_tags = format!("<body>\n");
        
        for tag in &self.body{
            match tag { 
                HtmlBody::H1(value) => {body_tags.push_str(format!("<h1>{value}</h1>\n").as_str())}
                HtmlBody::P(value) => {body_tags.push_str(format!("<p>{value}</p>\n").as_str())}
                HtmlBody::Img(value) => {body_tags.push_str(format!("<img src={} width={} height={} />\n", value.src, value.width, value.height).as_str())}
            }
        };

        body_tags.push_str("</body>");
        HtmlString(body_tags)
    }

    pub fn create_page(&self) -> HtmlString {
        let head_string:&HtmlString = &self.parse_head();
        let parse_body:&HtmlString = &self.parse_body();
        HtmlString(format!("<html>{} \n {}\n</html>", head_string.0, parse_body.0 ))
    }
}
