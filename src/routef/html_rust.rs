use std::{
    collections::{hash_map::Values, HashMap},
    fmt::format,
    fs::{self, read_to_string},
};

pub enum HtmlHead {
    TAG(String),
}

pub struct Img {
    pub src: String,
    pub width: i32,
    pub height: i32,
}

pub enum HtmlBody {
    H1(String),
    P(String),
    Img(Img),
    FileWithProps(String, HashMap<String, Option<String>>),
}

pub enum CSS {
    File(String),
}

pub struct HtmlString(pub String);

pub struct Page {
    pub head: Vec<HtmlHead>,
    pub body: Vec<HtmlBody>,
    pub css: Vec<CSS>,
}

pub fn read_html_add_props(htmlFile: String, props: HashMap<String, String>) -> String {
    let open_file = fs::read_to_string(htmlFile).expect(panic!("HTML_FILE"));
    let html = open_file.to_string();
    let html = format!("<div id='frouter_props'>{:#?}</div>", props) + open_file.as_str();
    return html;
}

impl Page {
    pub fn new() -> Page {
        Page {
            head: Vec::new(),
            body: Vec::new(),
            css: Vec::new(),
        }
    }

    pub fn parse_head(&self) -> HtmlString {
        let mut headtags = format!("\n<head>\n");

        for tag in &self.head {
            match tag {
                HtmlHead::TAG(value) => headtags.push_str(format!("{value} \n").as_str()),
            }
        }

        headtags.push_str("</head>");
        HtmlString(headtags)
    }

    pub fn parse_body(&self) -> HtmlString {
        let mut body_tags = format!("<body>\n");

        for tag in &self.body {
            match tag {
                HtmlBody::FileWithProps(file, props) => {
                    body_tags += html_with_params(file.to_owned(), props.to_owned())
                        .0
                        .as_str();
                }
                HtmlBody::H1(value) => body_tags.push_str(format!("<h1>{value}</h1>\n").as_str()),
                HtmlBody::P(value) => body_tags.push_str(format!("<p>{value}</p>\n").as_str()),
                HtmlBody::Img(value) => body_tags.push_str(
                    format!(
                        "<img src={} width={} height={} />\n",
                        value.src, value.width, value.height
                    )
                    .as_str(),
                ),
            }
        }

        body_tags.push_str("</body>");
        HtmlString(body_tags)
    }

    pub fn parse_css(&self) -> HtmlString {
        let mut current_css = String::new();
        current_css.push_str("<style>");

        for css in &self.css {
            match css {
                CSS::File(value) => {
                    let open_file = fs::read_to_string(value);
                    match open_file {
                        Ok(value) => current_css.push_str(value.as_str()),
                        Err(value) => panic!("error"),
                    }
                }
            }
        }

        current_css.push_str("</style>");
        HtmlString(current_css)
    }

    pub fn create_page(&self) -> HtmlString {
        let head_string: &HtmlString = &self.parse_head();
        let parse_body: &HtmlString = &self.parse_body();
        let parse_css: &HtmlString = &self.parse_css();
        HtmlString(format!(
            "<html>{} \n {}\n {} \n</html>",
            head_string.0, parse_body.0, parse_css.0
        ))
    }
}

pub fn add_param(value: String, params: HashMap<String, Option<String>>) -> String {
    //add_param in between opening and closing  clone()
    let mut return_value = value.clone();
    let mut index_into = 0;
    let matching_params = return_value.matches("}").count();

    for i in 0..matching_params {

        let opening = return_value.find(|x| x == '{').unwrap_or(0);
        let closing = return_value.find(|x| x == '}').unwrap_or(0);


        if opening > 0 && closing > opening {
            let mut value = return_value
                .get(opening..closing)
                .expect("Couldn't get the param inside the string")
                .replace("}", "")
                .replace("{", "");


            let get_param = params.get(&value).expect(
                format!("Found a param in your html that doesn't exist in your path {value}")
                    .as_str(),
            );

            return_value.replace_range(
                opening..closing + 1,
                get_param
                    .to_owned()
                    .unwrap_or("couldn't find this param".to_string())
                    .as_str(),
            );
        };
    }

    return_value
}

pub fn html_with_params(file: String, params: HashMap<String, Option<String>>) -> HtmlString {
    let open_file =
        fs::read_to_string(file).expect("Couldn't read file {file} are you sure the file exists?");
    let mut html_string: String = String::new();

    html_string += add_param(open_file.to_owned(), params.to_owned()).as_str();

    HtmlString(html_string)
}
