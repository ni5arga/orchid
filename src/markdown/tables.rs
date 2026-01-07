use regex::Regex;

pub fn transform(html: &str) -> String {
    let re_open = Regex::new(r"<table>").unwrap();
    re_open.replace_all(html, "<table class=\"md-table\">")
        .to_string()
}
