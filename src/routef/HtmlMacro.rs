#[macro_export]
macro_rules! css {
    ($($css:expr),*) => {
        {
            let mut temp_text = Vec::new();
            $(
                temp_text.push($css.to_string());
            )*
            println!("{:#?}", temp_text);
        }
    };
}
