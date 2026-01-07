use regex::Regex;

pub fn transform(html: &str) -> String {
    let re = Regex::new(r#"<img(?P<attrs>[^>]*?)(?P<self>\s*/?)>"#).unwrap();
    re.replace_all(html, |caps: &regex::Captures| {
        let attrs_raw = caps.name("attrs").map(|m| m.as_str()).unwrap_or("");
        let self_raw = caps.name("self").map(|m| m.as_str()).unwrap_or("");

        let mut attrs = attrs_raw.to_string();
        if attrs.is_empty() {
            attrs.push(' ');
        } else if !attrs.ends_with(' ') {
            attrs.push(' ');
        }

        let mut inject = String::new();
        if !attrs.contains("loading=") && !attrs_raw.contains("loading=") {
            inject.push_str("loading=\"lazy\" ");
        }
        if !attrs.contains("class=") && !attrs_raw.contains("class=") {
            inject.push_str("class=\"md-image\" ");
        }

        // Trim extra trailing space
        if inject.ends_with(' ') {
            inject.pop();
        }

        let self_closing = self_raw.trim().ends_with('/') || attrs.trim_end().ends_with('/');

        if inject.is_empty() {
            if self_closing {
                format!("<img{} />", attrs.trim_end())
            } else {
                format!("<img{}>", attrs.trim_end())
            }
        } else {
            let mut final_attrs = attrs;
            if !final_attrs.ends_with(' ') {
                final_attrs.push(' ');
            }
            final_attrs.push_str(&inject);
            if self_closing {
                format!("<img{} />", final_attrs.trim_end())
            } else {
                format!("<img{}>", final_attrs.trim_end())
            }
        }
    })
    .to_string()
}
